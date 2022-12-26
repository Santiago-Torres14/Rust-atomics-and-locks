use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        // implicitly moves numbers to this thread
        for n in numbers {
            println!("{n}");
        }
    }).join().unwrap();

    // This fails because the value was already move inside the previous thread
    //let handle_two = thread::spawn(move || {
    //    for n in numbers {
    //        println!("{n}");
    //   }
    //}).join().unwrap();
}
