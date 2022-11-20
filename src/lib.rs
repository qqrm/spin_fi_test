#![feature(test)]
#![allow(dead_code)]
extern crate test;

use rayon::prelude::*;
const MAX_ITER: usize = 8;

fn split_computation<F, T>(min_split_size: usize, data: Vec<T>, f: F) -> Vec<T>
where
    F: Fn(T) -> T,
    F: Sync + Send,
    T: Copy + Sync + Send,
{
    if min_split_size > data.len() {
        return data.iter().map(|el| f(*el)).collect();
    } else {
        return data.par_iter().map(|el| f(*el)).collect();
    }
}

fn is_even(n: u64) -> bool {
    n % 2 == 0
}

/// Algorithm from task
fn calc_num(mut num: u64) -> u64 {
    for i in 0..MAX_ITER {
        if 1 == num {
            return i as u64;
        }

        if is_even(num) {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let data = vec![1, 2, 3, 100];
        let expected_res = vec![0, 1, 7, 88];

        let res = split_computation(10000usize, data, calc_num);

        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_many_numbers() {
        const MIN_SPLIT_SIZE: usize = 2;
        let data = (1u64..1_000_001).collect();

        let res = split_computation(2000000usize, data, calc_num);

        assert_eq!(res.len(), 1000000usize);
    }

    #[test]
    fn test_mt_many_numbers() {
        const MIN_SPLIT_SIZE: usize = 2;
        let data = (1u64..1_000_001).collect();

        let res = split_computation(10000usize, data, calc_num);

        assert_eq!(res.len(), 1000000usize);
    }

    use test::Bencher;

    #[bench]
    fn bench_mt_many_numbers(b: &mut Bencher) {
        b.iter(|| test_mt_many_numbers());
    }

    #[bench]
    fn bench_many_numbers(b: &mut Bencher) {
        b.iter(|| test_many_numbers());
    }
}
