use std::{env, convert::TryInto, time::Instant};
use hex;
use regex::Regex;
use sha2::{Sha256, Digest};

fn main() {
    let now: Instant = Instant::now();
    let re: Regex = Regex::new(r"^[a-fA-F0-9]{64}").unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No hash provaded!")
    } else if !re.is_match(&args[1]) {
        panic!("Provided argument is not hash!")
    }
    let hash: [u8; 32] = hex::decode(&args[1]).unwrap().as_slice()
        .try_into().unwrap();
    let mut s: Vec<u8> = vec![32];
    let mut flag: bool = false;
    let mut iter_flag: bool = false;

    while !flag {
        for mut i in 0..s.len() {
            if iter_flag {
                i = 0;
                iter_flag = false;
            }
            for _x in 0..95 {
                let h: [u8; 32] = <[u8; 32]>::from(Sha256::digest(&s));
                println!("Hash: {} for pass: {}", hex::encode(h),
                     String::from_utf8(s.to_vec()).unwrap());
                if hash == h {
                    flag = true;
                    break;
                } else {
                    s[i] += 1;
                }
            }
            if flag {
                break;
            } else if s.ends_with(&[127]) {
                s.fill(32);
                s.push(32);
                iter_flag = true;
            } 
            while let Some(p) = s.iter().rposition(|&x| x == 127_u8) {
                s[p + 1] += 1;
                if s.ends_with(&[127]) {
                    s.fill(32);
                    s.push(32);
                } else {
                    for j in 0..p + 1 {
                        s[j] = 32;
                    }
                }
                iter_flag = true;
            }
        }
    }
    println!("Password is: \"{}\"", String::from_utf8(s).unwrap());
    println!("Time: {} milliseconds", now.elapsed().as_millis());
}