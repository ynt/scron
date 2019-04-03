# Scron
A Scron Expression Parser in Rust
### Fork from github.com/zslayton/cron

# Usage
```rust
extern crate scron;
use scron::Task;
use scron::Tasks;
use std::{thread,time};
use std::thread::sleep;


fn main() {
    thread::spawn(move || {
        let task = Task::new("gen data".to_owned(), "* 13 23 * * *", || {
            one_task();
        });

        one_task();

        let mut tasks: Tasks = Tasks { task: vec![task] };
        tasks.run();
    });

    loop {
        sleep(time::Duration::from_millis(100))
    }
}

fn one_task() {
    println!("hohoho");
}
```

# scron
```rust
extern crate scron;
extern crate chrono;

use scron::Schedule;
use chrono::Utc;
use std::str::FromStr;

fn main() {
  //               sec  min   hour   day of month   month   day of week   year
  let expression = "0   30   9,12,15     1,15       May-Aug  Mon,Wed,Fri  2018/2";
  let schedule = Schedule::from_str(expression).unwrap();
  println!("Upcoming fire times:");
  for datetime in schedule.upcoming(Utc).take(10) {
    println!("-> {}", datetime);
  }
}

/*
Upcoming fire times:
-> 2018-06-01 09:30:00 UTC
-> 2018-06-01 12:30:00 UTC
-> 2018-06-01 15:30:00 UTC
-> 2018-06-15 09:30:00 UTC
-> 2018-06-15 12:30:00 UTC
-> 2018-06-15 15:30:00 UTC
-> 2018-08-01 09:30:00 UTC
-> 2018-08-01 12:30:00 UTC
-> 2018-08-01 15:30:00 UTC
-> 2018-08-15 09:30:00 UTC
*/
```
