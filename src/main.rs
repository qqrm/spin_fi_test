use std::collections::HashMap;
use std::collections::VecDeque;
use std::vec;

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

        // dbg!(&next_deq);

        current_deq = next_deq;
    }

    map
}

fn calc_num(mut num: u64, k: u64, map: &HashMap<u64, u64>) -> u64 {
    for current_iter_count in 0..k {
        if is_even(num) {
            num = num / 2;
        } else {
            num = num * 3 + 1;
        }
    }
    dbg!(&num);

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
            None => calc_num(*num, k, &map),
        };
        map.insert(*num, val);
        res[i] = val;
    }

    res
}

fn main() {}

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
}
