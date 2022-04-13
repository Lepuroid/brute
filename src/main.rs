use std::env;
use regex::Regex;
use sha2::{Sha256, Digest};
use hex;
use std::convert::TryInto;


fn vec_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}


fn main() {
    let re = Regex::new(r"^[A-F0-9]{64}").unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("PANIC!!!")
    }
    if !re.is_match(&args[1]) {
        panic!("PANIC!!!")
    }
    let hash = vec_to_arr::<u8, 32>(hex::decode(&args[1]).unwrap());

    let mut hasher = Sha256::new();
    hasher.update(b"hello world");
    let res = hasher.finalize();

    // arr from generic array

    if hash == res{
        println!("{:?}", hash);
        println!("{:?}", res);
    }
}