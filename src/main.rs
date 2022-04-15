use std::env;
use regex::Regex;
use sha2::{Sha256, Digest};
use hex;
use std::convert::TryInto;


fn inc_char(mut c: char) -> char {
    if c < '~' {
        c = ((c as u8) + 1) as char;
    } else {
        c = '!';
    }
    c
}


fn main() {
    let re: Regex = Regex::new(r"^[A-F0-9]{64}").unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || !re.is_match(&args[1]){
        panic!("PANIC!!!")
    }
    let hash: [u8; 32] = hex::decode(&args[1]).unwrap().as_slice().try_into().unwrap();
    let mut c = '!';
    let mut s = String::from(c);
    let mut samp = <[u8; 32]>::from(Sha256::digest(&s));

    while !(hash == samp){
        c = inc_char(c);
        s.pop();
        s.push(c);
        samp = <[u8; 32]>::from(Sha256::digest(&s));
        if c == '~' {
            s.push('!');
        }
        println!("{}", s);
    }
    println!("{}", "The End!");
}