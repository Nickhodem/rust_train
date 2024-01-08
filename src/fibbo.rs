fn main() {
    let number = 30;
    let f = fib(number);
    for i in 0..number {
        println!("fib {i} is {}", fib(i));
    }
}

fn fib(n: i32) -> i32{
    let mut last_result = 1;
    let mut second_last = 0;
    if n == 0{return 0};
    for i in 1..n {
        let result = last_result + second_last;
        second_last = last_result;
        last_result = result;
    }
    return last_result;
}