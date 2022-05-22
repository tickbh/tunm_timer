use crate::{now_microsecond, Handler, Factory, RetTimer};
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use rbtree::RBTree;
use std::{thread, time};


#[derive(PartialEq, Eq)]
struct TreeKey(u64, u64);

pub struct Timer<F: Factory> {
    timer_queue: RBTree<TreeKey, Handler<F>>,
    time_maps: HashMap<u64, u64>,
    time_id: u64,
    time_max_id: u64,
    trigger_step: u64,
    shutdown: bool,
}

impl Ord for TreeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 != other.0 {
            return self.0.cmp(&other.0);
        }
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for TreeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<F:Factory> Timer<F> {
    pub fn new(trigger_step: u64) -> Timer<F> {
        Timer {
            timer_queue: RBTree::new(),
            time_maps: HashMap::new(),
            time_id: 0,
            time_max_id: u32::MAX as u64,
            trigger_step: trigger_step,
            shutdown: false,
        }
    }

    pub fn get_max_id(&self) -> u64 {
        self.time_max_id
    }

    pub fn set_max_id(&mut self, max_id : u64) {
        self.time_max_id = max_id;
    }


    pub fn get_trigger_step(&self) -> u64 {
        self.trigger_step
    }

    pub fn set_trigger_step(&mut self, trigger_step : u64) {
        self.trigger_step = trigger_step;
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn set_shutdown(&mut self, shutdown: bool) {
        self.shutdown = shutdown;
    }


    /// 添加定时器, 非定时器, 通常是重复定时器结束后进行的调用
    pub fn add_timer(&mut self, mut handle: Handler<F>) -> u64 {
        if handle.tick_step == 0 && handle.tick_ms == 0 {
            return 0;
        }
        if handle.time_id == 0 {
            handle.time_id = self.calc_new_id();
        };
        let time_id = handle.time_id;
        if handle.tick_step != 0 {
            if handle.at_once {
                handle.tick_ms = now_microsecond();
                handle.at_once = false;
            } else {
                handle.tick_ms = now_microsecond() + handle.tick_step;
            }
        }
        self.time_maps.insert(time_id, handle.tick_ms);
        self.timer_queue.insert(
            TreeKey(handle.tick_ms, time_id),
            handle,
        );
        time_id
    }

    /// 根据定时器的id删除指定的定时器
    pub fn del_timer(&mut self, time_id: u64) -> Option<Handler<F>> {
        if !self.time_maps.contains_key(&time_id) {
            return None;
        }
        let key = TreeKey(self.time_maps[&time_id], time_id);
        self.time_maps.remove(&time_id);
        self.timer_queue.remove(&key)
    }

    pub fn is_empty(&self) -> bool {
        self.timer_queue.len() == 0
    }

    /// 取出时间轴最小的一个值
    pub fn tick_first(&self) -> Option<u64> {
        self.timer_queue
            .get_first()
            .map(|(key, _)| Some(key.0))
            .unwrap_or(None)
    }

    /// 判断到指定时间是否有小于该指定值的实例
    pub fn tick_time(&mut self, tm: u64) -> Option<u64> {
        if tm < self.tick_first().unwrap_or(tm + 1) {
            return None;
        }
        if let Some((key, mut handle)) = self.timer_queue.pop_first() {
            let is_remove = match handle.factory.on_trigger(self, key.1) {
                RetTimer::Continue => {
                    if handle.tick_step == 0 {
                        true
                    } else {
                        false
                    }
                }
                RetTimer::Ok => {
                    if handle.tick_step == 0 {
                        true
                    } else {
                        !handle.is_repeat
                    }
                }
                RetTimer::Over => {
                    true
                }
            };
            if is_remove {
                self.time_maps.remove(&key.1);
            } else {
                self.add_timer(handle);
            }
            Some(key.1)
        } else {
            None
        }
    }

    /// 取出不冲突新的定时器id, 如果和已分配的定时器id重复则继续寻找下一个
    fn calc_new_id(&mut self) -> u64 {
        loop {
            self.time_id = self.time_id.overflowing_add(1).0;
            if self.time_id > self.time_max_id {
                self.time_id = 1;
            }
            if self.time_maps.contains_key(&self.time_id) {
                continue;
            }
            break;
        }
        self.time_id
    }

    pub fn run_loop_timer(&mut self) {
        let mut last_trigger_time = now_microsecond();
        while !self.shutdown {
            if self.trigger_step > 0 {
                let escape = now_microsecond() - last_trigger_time;
                if self.trigger_step > escape {
                    println!("will sleep time = {:?}", self.trigger_step - escape);
                    let trigger_sleep = time::Duration::from_micros(self.trigger_step - escape);
                    thread::sleep(trigger_sleep);
                }
            }
            let now = now_microsecond();
            while let Some(_) = self.tick_time(now) {
            }
            last_trigger_time = now_microsecond();
        }
    }
}

impl<F:Factory> fmt::Debug for Timer<F>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, handle) in self.timer_queue.iter() {
            let _ = writeln!(f, "{}", handle);
        }
        write!(f, "")
    }
}