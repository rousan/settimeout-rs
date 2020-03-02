use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

/// It implements the [`Future`](/std/future/trait.Future.html) trait.
///
/// # Examples
///
/// Create a [`Future`](/std/future/trait.Future.html) to be ready after some point:
///
/// ```
/// use std::time::Duration;
/// use settimeout::set_timeout;
/// use futures::executor::block_on;
///
/// async fn foo() {
///   println!("The Future will be ready after some time");
///   set_timeout(Duration::from_secs(5)).await;
///   println!("Now, it is ready");
/// }
///
/// block_on(foo());
/// ```
#[derive(Debug)]
pub struct Timer {
  state: Arc<Mutex<TimerState>>,
  duration: Duration,
}

impl Timer {
  /// Creates a new instance of `Timer` to be used as a `impl Future`.
  pub fn new(duration: Duration) -> Timer {
    Timer {
      state: Arc::new(Mutex::new(TimerState {
        timed_out: false,
        poll_called_already: false,
      })),
      duration,
    }
  }

  fn spawn_timer_thread(&self, cx: &mut Context<'_>) {
    let duration = self.duration;
    let state = Arc::clone(&self.state);
    let waker = cx.waker().clone();

    thread::spawn(move || {
      thread::sleep(duration);
      let mut state = state
        .lock()
        .expect("Couldn't lock the Timer state inside the Timer thread");
      (*state).timed_out = true;
      waker.wake();
    });
  }
}

impl Future for Timer {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let mut state = self.state.lock().expect("Couldn't lock the Timer state");

    if state.timed_out {
      Poll::Ready(())
    } else if state.poll_called_already {
      Poll::Pending
    } else {
      (*state).poll_called_already = true;
      self.spawn_timer_thread(cx);
      Poll::Pending
    }
  }
}

#[derive(Debug)]
struct TimerState {
  timed_out: bool,
  poll_called_already: bool,
}

/// It returns an implementation of [`Future`](/std/future/trait.Future.html) trait.
///
/// # Examples
///
/// Create a [`Future`](/std/future/trait.Future.html) to be ready after some point:
///
/// ```
/// use std::time::Duration;
/// use settimeout::set_timeout;
/// use futures::executor::block_on;
///
/// async fn foo() {
///   println!("The Future will be ready after some time");
///   set_timeout(Duration::from_secs(5)).await;
///   println!("Now, it is ready");
/// }
///
/// block_on(foo());
/// ```
pub fn set_timeout(duration: Duration) -> impl Future {
  Timer::new(duration)
}
