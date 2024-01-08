fn main() {
//     Enum
    let c = Currency::GBP;
    println!("{:?}", c as i32);
}
#[derive(Debug, Clone)]
enum Currency{
    EUR,
    PLN,
    GBP,
}

struct Money{
    value: f64,
    currency: Currency,
}
