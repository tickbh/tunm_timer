
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

pub trait Factory : Sized {
    fn on_trigger(&self, timer: &mut Timer<Self>, id: u64) -> RetTimer;
}


// impl<F, H> Factory for F
// where
//     H: Handler,
//     F: FnMut() -> H,
// {
//     type Handler = H;

//     fn timer_made(&mut self) -> H {
//         self()
//     }
// }


pub fn now_micro() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as u64 * 1000u64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as u64;
    ms
}