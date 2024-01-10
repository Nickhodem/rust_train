use std::mem;
use crate::record::{DataBase, HumanRecord, Record};

mod record;

fn main() {

    let hs = HumanRecord{
        id: 111,
        first_name: String::from("Jan"),
        last_name: String::from("Kowal"),
        is_active: true,
        age: 66,
    };

    println!("{:?}", hs);

    let s = hs.to_record();

    let bytes = s.to_bytes();

    println!("{:?}", bytes);

    let strc = Record::from_bytes(bytes);
    println!("{:?}", strc);

    let hss = strc.to_human();

    println!("{:?}", hss);

    let mut  database = DataBase::new(String::from("database.sql"));

    database.put(hs);

    let hs = HumanRecord{
        id: 256,
        first_name: String::from("Niko"),
        last_name: String::from("Pan"),
        is_active: false,
        age: 31,
    };
    database.put(hs);

    database.get(256);

    database.delete(111);



}

