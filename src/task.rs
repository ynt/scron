use super::Schedule;
use chrono::prelude::*;
use std::str::FromStr;
use std::thread::sleep;

pub struct Task {
    pub name: String,
    pub spec_str: String,
    pub func: fn(),
    pub spec: Schedule,
    pub prev: DateTime<Utc>,
    pub next: DateTime<Utc>,
    pub err_list: Vec<String>,
    pub err_lmit: i32,
}

impl Task {
    pub fn new(name: String, spec_str: &str, func: fn()) -> Self {
        let spec;
        if let Ok(result) = Schedule::from_str(spec_str) {
            spec = result
        } else {
            spec = Schedule::from_str("*").unwrap();
        }

        let now = Utc::now();
        let next;
        if let Some(result) = spec.upcoming(Utc).next() {
            next = result;
        } else {
            next = now;
        }

        Task {
            name: name,
            spec_str: spec_str.to_owned(),
            spec: spec,
            func: func,
            prev: now,
            next: next,
            err_list: vec![String::new()],
            err_lmit: 100,
        }
    }
}

pub struct Tasks {
    pub task: Vec<Task>,
}

// to do new a empty Tasks
impl Tasks {
    pub fn new(task: Task) -> Self {
        Tasks { task: vec![task] }
    }

    pub fn add_task(&mut self, task: Task) {
        self.task.push(task);
    }

    pub fn run(&mut self) {
        self.task[0].next = self.task[0].spec.after(&self.task[0].next).next().unwrap();
        loop {
            if self.task[0].next <= Utc::now() {
                (self.task[0].func)();
                self.task[0].next = self.task[0].spec.after(&self.task[0].next).next().unwrap();
                // thread::spawn(|| (task.func)());
            }

            sleep(time::Duration::SECOND)
        }
    }
}
