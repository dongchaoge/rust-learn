use std::thread;
use std::time::Duration;

pub fn run() {
    let handles = thread::spawn(|| {
        for i in 1..5 {
            println!("Spawn thread: {}", i);
            thread::sleep(Duration::from_secs(1))
        }
    });
    // if placed here, the main thread will be blocked until the sub thread finishes executing.
    // handles.join().unwrap();
    for i in 1..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_secs(1));
    }
    handles.join().unwrap();
}
