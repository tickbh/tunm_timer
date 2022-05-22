use crate::{now_micro, Handler, Factory, RetTimer};
use std::fmt;
use std::cmp::{Ord, Ordering};
use std::collections::HashMap;
use rbtree::RBTree;

#[derive(PartialEq, Eq)]
struct TreeKey(u64, u64);

pub struct Timer<F: Factory> {
    timer_queue: RBTree<TreeKey, Handler<F>>,
    time_maps: HashMap<u64, u64>,
    time_id: u64,
    time_max_id: u64,
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
    pub fn new(time_max_id: u64) -> Timer<F> {
        Timer {
            timer_queue: RBTree::new(),
            time_maps: HashMap::new(),
            time_id: 0,
            time_max_id: time_max_id,
        }
    }

    /// 添加定时器, 非定时器, 通常是重复定时器结束后进行的调用
    pub fn add_timer(&mut self, mut handle: Handler<F>) -> u64 {
        if handle.tick_step == 0 {
            return 0;
        }
        if handle.time_id == 0 {
            handle.time_id = self.calc_new_id();
        };
        let time_id = handle.time_id;
        if handle.at_once {
            handle.tick_ms = now_micro() + handle.tick_step;
            handle.at_once = false;
        } else {
            handle.tick_ms = now_micro();
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
        if let Some((key, handle)) = self.timer_queue.pop_first() {
            let is_remove = match handle.factory.on_trigger(self, key.1) {
                RetTimer::Continue => {
                    false
                }
                RetTimer::Ok => {
                    handle.is_repeat
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
}

impl<F:Factory> fmt::Debug for Timer<F>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, handle) in self.timer_queue.iter() {
            let _ = writeln!(f, "{}", handle);
        }
        write!(f, "")
    }
}