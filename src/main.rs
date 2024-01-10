use std::thread;
use std::time::Duration;

fn main() {
    let numbers = vec![1,2,3];

    let result = thread::Builder::new()
        .name("other".to_string())
        .spawn(move || task(&numbers));

    println!("Before join thread {:?}", thread::current());

    match result {
        Ok(handle) => _ = handle.join(),
        Err(error) => println!("Error {}", error)
    };

    println!("After joint {:?}", thread::current());
}

fn task(numbers: &Vec<i32>){
    println!("Numbers: {:?}", numbers);
    for index in 1.. 10 {
        println!("Index: {}, thread {:?}", index, thread::current());
        thread::sleep(Duration::from_secs(1));
    }
}