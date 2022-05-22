
use std::time::{SystemTime, UNIX_EPOCH};
extern crate rbtree;
use std::fmt;
mod timer;

pub use timer::Timer;

pub enum RetTimer {
    Ok,
    Continue,
    Over,
}

pub struct Handler<F>
where
    F: Factory,
{
    factory: F,
    time_id: u64,
    tick_ms: u64,
    tick_step: u64,
    is_repeat: bool,
    at_once: bool,
}



impl<F: Factory> fmt::Display for Handler<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "key({}, {}), step({}), is_repeat({}), at_once({})", self.tick_ms, self.time_id, self.tick_step, self.is_repeat, self.at_once)
    }
}

impl<F:Factory> Handler<F> {
    pub fn new_step(factory: F, tick_step: u64, is_repeat: bool, at_once: bool) -> Handler<F> {
        debug_assert!(tick_step > 0, "Step Mode Must Big 0s");
        let tick_step = ::std::cmp::max(1, tick_step);
        Handler {
            factory,
            time_id:0,
            tick_ms:0u64,
            tick_step,
            is_repeat,
            at_once,
        }
    }

    pub fn new_at(factory: F, tick_ms: u64) -> Handler<F> {
        Handler {
            factory,
            time_id:0,
            tick_ms:tick_ms,
            tick_step:0,
            is_repeat:false,
            at_once:false,
        }
    }
}

pub trait Factory : Sized {
    fn on_trigger(&mut self, timer: &mut Timer<Self>, id: u64) -> RetTimer;
}


pub fn now_microsecond() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as u64 * 1000_000u64 + (since_the_epoch.subsec_nanos() as f64 / 1_000.0) as u64;
    ms
}