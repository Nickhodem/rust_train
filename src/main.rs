use std::mem;
use crate::record::{HumanRecord, Record};

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
}

