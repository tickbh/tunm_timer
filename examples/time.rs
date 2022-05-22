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
        println!("on_trigger = {:} self.times = {}", id, self.times);
        self.times += 1;
        if self.times > 10 {
            if timer.is_empty() {
                timer.set_shutdown(true);
            }
            return RetTimer::Over;
        }
        println!("on_trigger = {:} self.times = {}", id, self.times);
        RetTimer::Ok
    }
}

fn main() {
    let mut timer = Timer::new(2000_000);
    let  time1 = timer.add_timer(Handler::new_step(
        RepeatTimeHandle{times:0}, 1000_000, true, true));
    println!("time == {}", time1);
    timer.run_loop_timer();
}