#![feature(test)]
#![allow(dead_code)]
extern crate test;

use rayon::prelude::*;

/// Processes each element of the data vector by either parallel or sequential computation 
/// based on the split_size.
/// 
/// # Arguments
/// 
/// * `split_size`: The threshold size above which parallel computation is used.
/// * `max_iter`: The maximum number of iterations for the function `f`.
/// * `data`: A vector of input data.
/// * `f`: A function that processes an element of data.
fn split_computation<F, T>(split_size: usize, max_iter: usize, data: Vec<T>, f: F) -> Vec<T>
where
    F: Fn(T, usize) -> T + Sync + Send,
    T: Copy + Sync + Send,
{
    if data.len() > split_size {
        data.par_iter().map(|el| f(*el, max_iter)).collect()
    } else {
        data.iter().map(|el| f(*el, max_iter)).collect()
    }
}

/// Checks if a number is even.
fn is_even(n: u64) -> bool {
    n % 2 == 0
}

/// Computes the number of steps to reduce the given number to 1 
/// following a specific algorithm, up to a maximum number of iterations.
fn calc_num(mut num: u64, max_iter: usize) -> u64 {
    for i in 0..max_iter {
        if num == 1 {
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
    fn test_simple_case() {
        let data = vec![1, 2, 3, 100];
        let expected_res = vec![0, 1, 7, 88];

        let res = split_computation(10000, 8, data, calc_num);

        assert_eq!(res, expected_res);
    }

    #[test]
    fn test_large_input_sequential() {
        let data = (1u64..1_000_001).collect::<Vec<_>>();

        let res = split_computation(1_000_002, 19, data, calc_num);

        assert_eq!(res.len(), 1_000_000);
    }

    #[test]
    fn test_large_input_parallel() {
        let data = (1u64..1_000_001).collect::<Vec<_>>();

        let res = split_computation(500_000, 19, data, calc_num);

        assert_eq!(res.len(), 1_000_000);
    }

    use test::Bencher;

    #[bench]
    fn bench_parallel_computation(b: &mut Bencher) {
        b.iter(|| test_large_input_parallel());
    }

    #[bench]
    fn bench_sequential_computation(b: &mut Bencher) {
        b.iter(|| test_large_input_sequential());
    }
}
