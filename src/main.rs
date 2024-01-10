use std::thread;
use std::time::Duration;

fn main() {
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