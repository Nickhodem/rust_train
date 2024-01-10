use std::{env, fs};
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args()
        .collect();
    let option = args[1].as_str();

    match option {
        "echo" => {
            let st = args[2..].join(" ");
            println!("{}", st);
        }
        "cat" => {
            println!{"echo file..."};
            let mut numbers = false;
            let mut number_no_blank = false;
            let mut it = args.clone().into_iter();
            for a in it{
                if a == "n"{
                    numbers = true;
                }
                if a == "b"{
                    number_no_blank=true;
                }
            }

            if args.len() >= 2{
                echo(&args[2], numbers, number_no_blank)
            }
        },
        "wc" => {
            wc(&args[2]);
        }
        "find" => {
            find(&args[2], &args[3]);
        }
        value=> {
            println!("Wrong parameters");
        }
    };
}

fn echo(string: &String, number: bool, number_no_blank: bool){
    let file = File::open(string)
        .unwrap();
    let reader = BufReader::new(file);
    for (index, line) in reader.lines().enumerate() {
        if let Ok(current_line) = line {
            if number{
                print!("{} ", index);
            }
            if number_no_blank{
                if current_line.len()>0{
                    print!("{} ", index);
                }
            }
            print!("{}\n", current_line)
        }
    }
}

fn wc(string: &String){
    let file = File::open(string)
        .unwrap();
    let reader = BufReader::new(file);
    let mut word_num = 0;
    let mut char_num =0;
    let mut byte_num = 0;
    let mut line_num = 0;
    for line in reader.lines(){
        if let Ok(current_line) = line {
            line_num += 1;
            byte_num += current_line.bytes().len();
            char_num += current_line.len();
            let vec: Vec<_> = current_line.split_whitespace().collect();
            word_num += vec.count();
        }
    }
    println!("lines: {}, words: {}, chars: {}, bytes: {}", line_num, word_num, char_num, byte_num);
}

fn find(pattern: &String, directory: &String){
    let re = Regex::new(pattern).unwrap();

    for entry_res in read_dir(directory).unwrap() {

        let entry = entry_res.unwrap();
        let file_name = entry.file_name();
        let file_name_buf = file_name.to_str().unwrap();

        let matching = re.is_match(file_name_buf);

        if matching {
            let p = entry.path();

            println!("{:?} : {}", file_name_buf, p.to_str().unwrap());
        }

        if entry.file_type().unwrap().is_dir(){
            let p = entry.path();
            find(pattern, &String::from(p.to_str().unwrap()));
        }
    }
}