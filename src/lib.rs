//! Contains some utility features to create a [`Future`](/std/future/trait.Future.html) implementation
//! to be used in any async function.
//!
//! # Examples
//!
//! Create a [`Future`](/std/future/trait.Future.html) to be ready after some point:
//!
//! ```
//! use std::time::Duration;
//! use settimeout::set_timeout;
//! use futures::executor::block_on;
//!
//! async fn foo() {
//!   println!("The Future will be ready after some time");
//!   set_timeout(Duration::from_secs(5)).await;
//!   println!("Now, it is ready");
//! }
//!
//! block_on(foo());
//! ```

pub use timer::set_timeout;
pub use timer::Timer;

mod timer;

#[cfg(test)]
mod tests {
    use super::*;
    use futures::executor::block_on;
    use std::time::Duration;

    #[test]
    fn set_timeout_should_wait() {
        block_on(async {
            set_timeout(Duration::from_secs(2)).await;
        });
    }

    #[test]
    fn timer_should_wait() {
        block_on(async {
            Timer::new(Duration::from_secs(2)).await;
        });
    }
}
