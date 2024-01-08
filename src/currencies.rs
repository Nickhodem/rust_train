mod structs;

use std::fmt::{Debug, Formatter};

fn main() {
    let mut pln = Money{name: String::from("PLN"), amount:100.0,};
    let pln2 = Money{name: String::from("PLN"), amount:25.0,};
    pln.add(&pln2);

    let euro = pln.convert(String::from("EURO"), 1.0/4.3475);
    println!("Euro {:#?}", euro);

    let usd = euro.convert(String::from("USD"), 1.0/1.09);
    println!("usd {:#?}", usd);

    println!("pln {:#?}", pln2);
}



#[derive(Debug, Clone)]
struct Money{
    amount: f32,
    name: String,
}

impl Money {
    fn add(&mut self, other: &Money){
        if self.name == other.name{
            self.amount += other.amount;
        }
        else{
            println!{"Cannot add {} to {}", self.name, other.name};
        }
    }

    fn convert(&self, currency: String, ratio: f32) -> Money {
        return Money{
            amount: self.amount * ratio,
            name: currency
        }
    }
}
