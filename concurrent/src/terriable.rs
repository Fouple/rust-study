use std::thread;
use std::time::Duration;

fn terriable() {
    let new_thread = thread::spawn(move || {
        thread::spawn(|| {
            loop {
                println!("die!!!");
            }
        })
    });

    new_thread.join().unwrap();
    println!("Child thread is finish");

    thread::sleep(Duration::from_secs(10));
}