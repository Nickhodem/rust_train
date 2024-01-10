use std::{mem, slice};
use std::cmp::min;
use std::intrinsics::copy_nonoverlapping;
use std::io::{Read, Write};
pub struct DataBase{
    filename: String,
}

const STRING_SIZE: usize = 10;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct Record{
    pub id: i64,
    pub first_name: [u8; STRING_SIZE],
    pub last_name: [u8; STRING_SIZE],
    pub is_active: bool,
    pub age: u8,
}
#[derive(Debug, Clone)]
pub struct HumanRecord{
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub is_active: bool,
    pub age: u8,
}

const RECORD_SIZE:usize =  mem::size_of::<Record>();

impl Record{
    pub fn to_bytes(&self) -> [u8; RECORD_SIZE]{
        let mut buffer: [u8; RECORD_SIZE] = [0; RECORD_SIZE];
        let mut byte_num = 0;

        for elem in self.id.to_be_bytes().iter(){
            buffer[byte_num] = *elem;
            byte_num+=1;
        }

        for elem in self.first_name.iter(){
            buffer[byte_num] = *elem;
            byte_num+=1;
        }
        for elem in self.last_name.iter(){
            buffer[byte_num] = *elem;
            byte_num+=1;
        }

        buffer[byte_num] = self.is_active.into();
        byte_num+=1;

        buffer[byte_num] = self.age;

        return buffer;
    }

    pub fn from_bytes(bytes: [u8; RECORD_SIZE]) -> Self{
        let mut rec = Record{
            id: 0,
            first_name: [0; STRING_SIZE],
            last_name: [0; STRING_SIZE],
            is_active: false,
            age: 0,
        };

        let id_size = mem::size_of::<i64>();
        rec.id  = i64::from_be_bytes(bytes[0.. id_size].try_into().unwrap());

        rec.first_name.clone_from_slice(&bytes[id_size.. STRING_SIZE+id_size]);

        rec.last_name.clone_from_slice(&bytes[id_size+STRING_SIZE.. id_size+ 2*STRING_SIZE]);

        rec.is_active = if bytes[id_size+STRING_SIZE*2 ] == 1 {true} else {false};

        rec.age = bytes[RECORD_SIZE-1].into();

        return rec;


    }

    pub fn to_human(&self) -> HumanRecord{
        let rec = HumanRecord{
            id: self.id,
            first_name: Self::arr_to_string(self.first_name.clone()),
            last_name: Self::arr_to_string(self.last_name.clone()),
            is_active: self.is_active,
            age: self.age,
        };

        return rec;
    }

    fn arr_to_string(arr: [u8; STRING_SIZE]) -> String{
        return  String::from_utf8(Vec::from(arr)).unwrap().replace("\0", "");
    }

}

impl HumanRecord{
    pub fn to_record(&self) -> Record{
        let rec = Record{
            id: self.id,
            first_name: self.string_to_arr(self.first_name.clone()),
            last_name: self.string_to_arr(self.last_name.clone()),
            is_active: self.is_active,
            age: self.age
        };
        return rec;
    }

    fn string_to_arr(&self, string: String) -> [u8; STRING_SIZE]{
        let bytes: &[u8] = string.as_bytes();

        let mut pad_bytes: [u8; STRING_SIZE] = [0; STRING_SIZE];

        for i in 0 .. min(8, bytes.len()){
            pad_bytes[i] = bytes[i];
        }

        return pad_bytes;
    }



}