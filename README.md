#tunm_timer
==========

简易定时器

```rust
extern crate tunm_timer;
use tunm_timer::{Factory, Timer, RetTimer, Handler};

struct TimeHandle;

impl tunm_timer::Factory for TimeHandle {
    fn on_trigger(&mut self, timer: &mut Timer<Self>, id: u64) -> RetTimer {
        println!("ontigger = {:}", id);
        RetTimer::Ok
    }
}

struct RepeatTimeHandle {
    times: u32,
}
impl tunm_timer::Factory for RepeatTimeHandle {
    fn on_trigger(&mut self, timer: &mut Timer<Self>, id: u64) -> RetTimer {
        self.times += 1;
        if self.times > 10 {
            return RetTimer::Over;
        }
        println!("ontigger = {:} self.times = {}", id, self.times);
        RetTimer::Ok
        // timer.add_timer(mut handle: Handler<F>)
    }
}

fn main() {
    println!("ok");
    let mut timer = Timer::new(u64::MAX);
    let  time1 = timer.add_timer(Handler::new_step(
        RepeatTimeHandle{times:0}, 1000_000, true, true));
    println!("time == {}", time1);
    loop {
        timer.tick_time(tunm_timer::now_microsecond());
        if timer.is_empty() {
            break;
        }
    }
}
```