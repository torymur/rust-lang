extern crate rand;

use crate::rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut v: Vec<u8> = Vec::new();
    let length = 15;
    fill_with_random(&mut v, length);
    println!("Random vector is {:?}", v);
    println!("Its mean is {}", mean(&v));
    println!("Its median is {}", median(&v));
    println!("Random vector is unchanged {:?}", v);
    println!("Its mode is {}", mode(&v));
}

fn fill_with_random(v: &mut Vec<u8>, length: u8) {
    let mut rng = rand::thread_rng();
    for _ in 0..length {
        v.push(rng.gen_range(0, 10));
    }
}

// average value
fn mean(v: &Vec<u8>) -> f64 {
    let total: u8 = v.iter().sum();
    total as f64 / v.len() as f64
}

// when sorted, the value in the middle position
fn median(v: &Vec<u8>) -> u8 {
    let middle = v.len() / 2;
    let mut copy_v = v.to_vec();
    copy_v.sort();
    println!("Sorted vector is {:?}", copy_v);
    copy_v[middle]
}

// the value that occurs most often
fn mode(v: &Vec<u8>) -> u8 {
    let mut stats: HashMap<u8, u8> = HashMap::new();
    for n in v.iter() {
        let count = stats.entry(*n).or_insert(0);
        *count += 1;
    }
    let mut key = 0;
    let mut value = 0;
    for (k, v) in &stats {
        if *v > value {
            key = *k;
            value = *v;
        }
    }
    key
}
