extern crate tunm_timer;
use tunm_timer::{Factory, Timer, RetTimer, Handler};

struct TimeHandle;

impl tunm_timer::Factory for TimeHandle {
    fn on_trigger(&self, timer: &mut Timer<Self>, id: u64) -> RetTimer {
        println!("ontigger = {:}", id);
        RetTimer::Ok
        // timer.add_timer(mut handle: Handler<F>)
    }
}

fn main() {
    println!("ok");
    let mut timer = Timer::new(u64::MAX);
    let  time = timer.add_timer(Handler::new_at(
        TimeHandle{},
        tunm_timer::now_micro() + 1000,
    ));
    println!("time == {}", time);
    loop {
        timer.tick_time(tunm_timer::now_micro() + 2000);
        if timer.is_empty() {
            break;
        }
    }
}