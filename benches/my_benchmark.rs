use criterion::{black_box, criterion_group, criterion_main, Criterion};

// fn fibonacci(n: u64) -> u64 {
//     match n {
//         0 => 1,
//         1 => 1,
//         n => fibonacci(n-1) + fibonacci(n-2),
//     }
// }

//use std::{convert::TryInto, time::Instant};
use std::convert::TryInto;
use hex;
use sha2::{Sha256, Digest};

fn brute(arg: String) {
//    let now: Instant = Instant::now();
    let hash: [u8; 32] = hex::decode(&arg).unwrap().as_slice()
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
            for _x in 0..95 as u8{
                let h: [u8; 32] = <[u8; 32]>::from(Sha256::digest(&s));
                // println!("Hash: {} for pass: {}", hex::encode(h),
                //         String::from_utf8(s.to_vec()).unwrap());
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
                continue;
            } 
            while let Some(p) = s.iter().rposition(|&x| x == 127_u8) {
                s[p + 1] += 1;
                iter_flag = true;
                if s.ends_with(&[127]) {
                    s.fill(32);
                    s.push(32);
                } else {
                    for j in 0..p + 1 {
                        s[j] = 32;
                    }
                }  
            }
        }
    }
    // println!("Password is: \"{}\"", String::from_utf8(s).unwrap());
    // println!("Time: {} milliseconds", now.elapsed().as_millis());
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("brute", |b| b.iter(|| brute(black_box(String::from("eb87821f650a847d70a85d4221ca46050b9bd2e421a185100dcfc43d9f93a3bb")))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

//Intel(R) Core(TM) i3-4160 CPU @ 3.60GHz 
//time:   [308.55 ms 309.01 ms 309.55 ms]

//(VM linux) AMD Ryzen 7 PRO 4750U
//time:   [265.57 ms 265.95 ms 266.37 ms]
