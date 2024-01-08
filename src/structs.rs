use std::fmt::{Debug, Formatter};

fn main() {

    let origin = Point(1,2);
    println!("x{}, y{}", origin.0, origin.1);

    let active = true;

    let mut account = Account{
        active,
        email: String::from("n@aptiv.com"),
        password: String::from("1234")
    };

    account.active = false;

    println!("{:#?}", account);

    // Copy parameters
    let mut account_2 = Account{
        email: String::from("w@aptiv.com"),
        ..account.clone()
    };

    println!("{:?}", account);
    println!("{:?}", account.password);
    let sq = Rectangle::square(10.0);
    println!("{:#?}, {:?}", sq, sq.area());

}


// unit struct
struct Directory;

struct Point(i32, i32);


#[derive(Debug, Clone)]
struct Account{
    active: bool,
    email: String,
    password: String,
}
#[derive(Debug, Clone)]
struct Rectangle{
    width: f32,
    height: f32,
}

impl Rectangle{
    fn area(&self) -> f32{
        self.width * self.height
    }

    fn square(size: f32) -> Self{
        Self{
            width: size,
            height: size,
        }
    }
}
