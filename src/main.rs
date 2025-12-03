use std::sync::Mutex;

fn main() {
    let mu = Mutex::new(10);
    {
        println!("Inside block...");
        let mut val = mu.lock().unwrap();
        println!("val = {:?}", mu);
        *val += 1;
    }
    println!("Outside block...");
    println!("val = {:?}", mu);

    let mu = Mutex::new(20);
    println!("{:?}", mu);
    let mut val = mu.lock().unwrap();
    *val += 1;
    println!("{:?}", mu);
    std::mem::drop(val);
    println!("{:?}", mu);
}
/*
Example demonstrating basic multi-threading in Rust.
use std::time::Duration;
use std::thread;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("From spawned thread - {}", i);
            thread::sleep(Duration::from_millis(2));
        }
        println!("Spawned thread finished");
    });
    for i in 1..3 {
        println!("From main thread - {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    println!("Main thread finished");
}
    // Example demonstrating basic multi-threading with thread join in Rust.
use std::time::Duration;
use std::thread;

fn main() {
    let h = thread::spawn(|| {
        for i in 1..10 {
            println!("From spawned thread - {}", i);
            thread::sleep(Duration::from_millis(2));
        }
        println!("Spawned thread finished");
    });
    for i in 1..3 {
        println!("From main thread - {}", i);
        thread::sleep(Duration::from_millis(2));
    }
    println!("Main thread finished");
    // wait for the spawned thread to finish
    h.join().unwrap();
}

    // Example demonstrating message passing between threads using channels in Rust.
    use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();
    let tx3 = tx1.clone();
    thread::spawn(move || {
        let num_vec = vec![1, 2, 3];
        for num in num_vec {
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });
    thread::spawn(move || {
        let num_vec = vec![4, 5, 6];
        for num in num_vec {
            tx2.send(num).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });
    thread::spawn(move || {
        let actors_vec = vec![7, 8, 9];
        for actor in actors_vec {
            tx3.send(actor).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });
    for received_val in rx {
        println!("Received : {}", received_val);
    }
}
 // Example demonstrating Mutex for shared state across threads in Rust.
 use std::sync::Mutex;

fn main() {
    let mu = Mutex::new(10);
    let mut val = mu.lock().unwrap();
    println!("val = {:?}", val);
    *val += 1;
    println!("val = {:?}", val);
}
*/
