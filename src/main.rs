#[allow(unused_assignments)]
#[allow(unused_variables)]
fn main() -> () {
    println!("Hello, world!");

    let mut x: i32 = 32;
    println!("x = {x}");
    x = 33;
    println!("x = {x}");
    let o = 100;

    //  Shadowing

    let x = 11.0;
    println!("x is now {x}");

    {
        let x = x as f32 * 3.1;
        println!("x is now {x}");
    }
    println!("x is now {x}");
    let x = add(2.5, 5.1);
    println!("x is now {x}");

    const MONTH_OF_THE_YEAR: i8 = 4;

}

fn add(var: f32, other_var: f32)->f32{
    return var + other_var;
}
