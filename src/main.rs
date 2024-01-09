use std::env;
use std::fs::{File, OpenOptions};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let budget = Budget::new(String::from("budget.txt"));

    // type=[deposit, withdrawn] amount = 2000.0 descrtiption="Kino"

    let args: Vec<String> = env::args()
        .collect();
    let option = args[1].as_str();

    match option {
         "show_summary" => {
            println!{"showing summary of budget"};
            budget.show_summary();
        },
        "deposit" => {
            println!("Registring deposit");
            let amount = budget.amount(&args[2]);
            let type_ = budget.type_(&args[3]);
            budget.deposit(amount, type_);
        },
        "withdrawn" => {
            println!("Registring withdrawn");
            let amount = budget.amount(&args[2]);
            let type_ = budget.type_(&args[3]);
            budget.withdrawn(amount, type_);
        },
        value=> {
            println!("Wrong parameters");
        }
    };


}

struct Budget{
    filename: String,
}

impl Budget{

    fn new(filename: String) -> Budget{
        if !Path::new("/etc/hosts").exists(){
        let test_file = File::create(filename.clone());
        }
        return Budget{filename: filename.clone()};
    }
    fn show_summary(&self){
        let mut sum = 0.0;
        let file = File::open(self.filename.clone())
            .unwrap();
        let reader = BufReader::new(file);
        for (index, line) in reader.lines().enumerate() {
            if let Ok(current_line) = line {
                println!("{}: {}", index + 1, current_line);
                let line_vec: Vec<_> = current_line.split_whitespace().collect();

                let amount = f64::from_str(line_vec[2]).unwrap();
                if line_vec[1] == "deposit"{
                    sum += amount;
                }
                else {
                    sum -= amount;
                }
            }
        }
        println!("Summary: {sum}");
    }

    fn deposit(&self, amount: f64, type_: String){
        self.write(String::from("deposit"), amount, type_);
    }

    fn withdrawn(&self, amount: f64, type_: String){
        self.write(String::from("withdrawn"), amount, type_);
    }

    fn amount(&self, var: & String) -> f64{
        let f = var.parse::<f64>().unwrap();
        println!("amount {}", f);
        return f;
    }

    fn type_(&self, var: & String) -> String{
        return var.clone();
    }

    fn write(&self, operation: String, amount: f64, type_: String){
        let test_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(self.filename.clone());

        let dt = Utc::now();
        let formatted = format!("{}", dt.format("%d/%m/%Y@%H:%M"));
        if let Ok(mut output_file) = test_file {
            writeln!(output_file, "{formatted}, {operation} {amount} {type_}").expect("panic message");
        }
    }
}