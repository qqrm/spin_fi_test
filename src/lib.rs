use std::collections::{HashMap, VecDeque};
use std::{thread, vec};

fn is_even(n: u64) -> bool {
    n % 2 == 0
}

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

fn calc(data: Vec<u64>, k: u64) -> Vec<u64> {
    let mut map = generate_map(k);
    println!("Map: {:?}", map);

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

fn calc_mt(data: Vec<u64>, k: u64, chunk_size: usize) -> Vec<u64> {
    let map = generate_map(k);
    println!("Map: {:?}", map);

    let mut chunks = Vec::new();
    for chunk in data.chunks(chunk_size) {
        let mut v = Vec::new();
        v.extend(chunk.iter().cloned());
        chunks.push(v);
    }

    let mut handles = Vec::new();

    for chunk in chunks {
        let mut copy_map = map.clone();
        let handle = thread::spawn(move || {
            let mut chunk_res = Vec::with_capacity(chunk_size);
            for num in chunk {
                let result_for_num = copy_map.get(&num);

                let val = match result_for_num {
                    Some(result) => *result,
                    None => calc_num(num, k),
                };
                copy_map.insert(num, val);
                chunk_res.push(val);
            }

            chunk_res
        });
        handles.push(handle);
    }

    let mut res = Vec::with_capacity(data.len());

    for handle in handles {
        let mut chunk_res = handle.join().unwrap();
        res.append(&mut chunk_res);
    }

    res
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

        let res = calc_mt(data, k, chunk_size);

        assert_eq!(res, expected_res);
    }
}
