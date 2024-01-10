use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let atomic_counter = Arc::new(Mutex::new(0));
    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _ in 1.. 1_000{
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            if let Ok(mut guard ) = counter.lock(){
                // to get into i32 into mutex
                *guard += 1;
                println!("Counter value {} {:?}", *guard, thread::current() );
                thread::sleep(Duration::from_millis(1000));
            }
        });
        handles.push(handle);

    }
    for handle in handles{
        _ = handle.join();
    }
    print!("Counter {} {:?}", *atomic_counter.lock().unwrap(), thread::current());
}

fn threads() {
    let numbers = vec![1,2,3];

    let result = thread::Builder::new()
        .name("other".to_string())
        .spawn(move || task(numbers));

    println!("Before join thread {:?}", thread::current());

    let values = match result {
        Ok(handle) => handle.join(),
        Err(error) => {
            println!("Error {}", error);
            Ok(vec![0])
        }
    }.expect("Failed");

    println!("After joint {:?}", thread::current());
    println!("num {}", values[0]);
}

fn task(numbers: Vec<i32>) -> Vec<i32>{
    println!("Numbers: {:?}", numbers);
    for index in 1.. 10 {
        println!("Index: {}, thread {:?}", index, thread::current());
        thread::sleep(Duration::from_secs(1));
    }
    return numbers;
}