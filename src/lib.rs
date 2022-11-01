#![feature(test)]
#![allow(dead_code)]
extern crate test;

use simple_error::SimpleError;
use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;
use std::{thread, vec};

fn is_even(n: u64) -> bool {
    n % 2 == 0
}

/// Precalc possible nums
fn generate_map(k: u64) -> HashMap<u64, u64> {
    let mut map = HashMap::new();

    let mut current_deq = VecDeque::new();
    current_deq.push_back(1u64);

    for current_iter_count in 0..k + 1 {
        let mut next_deq = VecDeque::new();

        while !current_deq.is_empty() {
            let num = current_deq.pop_front().unwrap();
            map.insert(num, current_iter_count);

            next_deq.push_back(num * 2);

            let possible_next = (num - 1) / 3;
            if !is_even(num - 1) && !is_even(possible_next) && !map.contains_key(&possible_next) {
                next_deq.push_back(possible_next);
            }
        }

        current_deq = next_deq;
    }

    map
}

/// Algorithm from task
fn calc_num(mut num: u64, k: u64) -> u64 {
    (0..k).for_each(|_| {
        if is_even(num) {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }
    });

    num
}

// Calc result single thread
fn calc(data: Vec<u64>, k: u64) -> Vec<u64> {
    let mut map = generate_map(k);

    let mut res = vec![0u64; data.len()];

    for (i, num) in data.iter().enumerate() {
        let result_for_num = map.get(num);

        let val = match result_for_num {
            Some(result) => *result,
            None => calc_num(*num, k),
        };
        map.insert(*num, val);
        res[i] = val;
    }

    res
}

// Calc result multithread
fn calc_mt(data: Vec<u64>, k: u64, chunk_size: usize) -> anyhow::Result<Vec<u64>> {
    let locked_map = RwLock::new(generate_map(k));

    let mut chunks = Vec::new();
    for chunk in data.chunks(chunk_size) {
        let mut v = Vec::new();
        v.extend(chunk.iter().cloned());
        chunks.push(v);
    }

    let res_lock = RwLock::new(Vec::with_capacity(data.len()));
    thread::scope(|s| {
        for chunk in chunks {
            s.spawn(|| {
                let mut chunk_res = Vec::with_capacity(chunk_size);

                for num in chunk {
                    if let Ok(mut map) = locked_map.write() {
                        let val = if let Some(val) = map.get(&num) {
                            *val
                        } else {
                            calc_num(num, k)
                        };

                        map.insert(num, val);
                        chunk_res.push(val);
                    }
                }

                if let Ok(mut res) = res_lock.write() {
                    res.append(&mut chunk_res);
                }
            });
        }
    });

    if let Ok(res) = res_lock.read() {
        let mut res = res.clone();
        res.sort();
        return Ok(res);
    }

    Err(SimpleError::new("Joining result error"))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let data = vec![1, 2, 3, 100];
        let expected_res = vec![0, 1, 7, 88];
        let k = 8;

        let res = calc(data, k);

        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_simple_mt() {
        let data = vec![1, 2, 3, 100];
        let expected_res = vec![0, 1, 7, 88];
        let k = 8;
        let chunk_size = 2;

        let res = calc_mt(data, k, chunk_size).unwrap();

        assert_eq!(res, expected_res);
    }

    use test::Bencher;

    #[bench]
    fn single(b: &mut Bencher) {
        b.iter(|| test_simple());
    }

    #[bench]
    fn mt_slow(b: &mut Bencher) {
        b.iter(|| test_simple_mt());
    }
}
