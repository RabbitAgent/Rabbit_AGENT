use governor::{Jitter, Quota, RateLimiter};
use std::num::NonZeroU32;

pub fn create_limiter() -> RateLimiter<NotKeyed, InMemoryState, DefaultClock> {
    let quota = Quota::per_second(NonZeroU32::new(1000).unwrap())
        .allow_burst(NonZeroU32::new(5000).unwrap());
    
    RateLimiter::direct_with_clock(quota, &DefaultClock::default())
}
