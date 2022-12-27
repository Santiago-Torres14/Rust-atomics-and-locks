use std::thread::{self, spawn};
use std::time;

fn main() {
    let numbers = vec![1, 2, 3];    

    // before our program ends, thread::scope calls join which awaits
    // until all the threads are done with their tasks
    thread::scope(|s| {
        s.spawn(|| {
          thread::sleep(time::Duration::from_millis(5000));
        });

        s.spawn(|| {
            // explicit borrow of an outside variable
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}
