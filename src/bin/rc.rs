use std::{rc::Rc, sync::Arc, thread, time};

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());

    let a = Arc::new([1, 2, 3]);

    let handle = thread::spawn(move || {
        let a = a.clone();
        for v in *a {
            println!("{v}");
        }
    });
    let handle_two = thread::spawn(move || {
        thread::sleep(time::Duration::from_secs(5));
        println!("second thread finished");
    });


    handle_two.join();
    handle.join();
}
