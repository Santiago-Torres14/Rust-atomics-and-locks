use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;

fn main() {
    let progress = AtomicUsize::new(0);

    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..25 {
                    progress.fetch_add(1, Relaxed);
                    thread::sleep(Duration::from_secs(1));
                }
            });
        }

        loop {
            let n = progress.load(Relaxed);
            println!("{}/100", n);
            if n == 100 { break; }
            thread::sleep(Duration::from_secs(1));
        }
    });
}
