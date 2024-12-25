use std::hash::Hash;
use std::ops::Add;

use hashbrown::HashMap;

fn parse_input(input: &str) -> Vec<u128> {
    input
        .lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() {
                return None;
            }

            l.parse().ok()
        })
        .collect()
}

pub fn sum_nth_secret_number(input: &str, n: usize) -> u128 {
    let nums = parse_input(input);
    nums.iter().map(|&num| get_nth_secret_number(num, n)).sum()
}

pub fn get_max_bananas(input: &str, n: usize) -> usize {
    let mut deltas_price_map = HashMap::new();

    let nums = parse_input(input);

    for num in nums {
        let mut curr_deltas_price_map = HashMap::new();
        let (prices, deltas) = get_n_prices_and_deltas(num, n);

        for (i, delta_window) in deltas.windows(4).enumerate() {
            if i + 4 >= prices.len() {
                break;
            }
            let price = prices[i + 4] as usize;
            let t = (
                delta_window[0],
                delta_window[1],
                delta_window[2],
                delta_window[3],
            );

            if curr_deltas_price_map.contains_key(&t) {
                continue;
            }
            curr_deltas_price_map.insert(t, price);
        }

        for (t, price) in curr_deltas_price_map {
            if let Some(curr_price) = deltas_price_map.get_mut(&t) {
                *curr_price += price;
            } else {
                deltas_price_map.insert(t, price);
            }
        }
    }

    deltas_price_map.values().copied().max().unwrap()
}

fn get_n_prices_and_deltas(num: u128, n: usize) -> (Vec<u8>, Vec<i32>) {
    let mut prices = Vec::with_capacity(n);
    let mut deltas = Vec::with_capacity(n - 1);
    let mut s = num;
    for _ in 0..n {
        let new_s = get_next_secret_number(s);
        let price = new_s % 10;
        let price = price as u8;

        if let Some(last) = prices.last() {
            let last = *last as i32;
            let price = price as i32;
            deltas.push(price - last);
        }

        prices.push(price);

        s = new_s;
    }

    (prices, deltas)
}

#[inline(always)]
fn get_next_secret_number(num: u128) -> u128 {
    let r = num * 64;
    let s = mix(num, r);
    let s = prune(s);

    let r = s / 32;
    let s = mix(s, r);
    let s = prune(s);

    let r = s * 2048;
    let s = mix(s, r);
    prune(s)
}

#[inline(always)]
fn get_nth_secret_number(num: u128, n: usize) -> u128 {
    let mut s = num;
    for _ in 0..n {
        s = get_next_secret_number(s);
    }

    s
}

#[inline(always)]
fn mix(s: u128, r: u128) -> u128 {
    s ^ r
}

#[inline(always)]
fn prune(s: u128) -> u128 {
    s % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_INPUT: &str = "1
10
100
2024";

    const TEST_MAX_BANANAS_INPUT: &str = "1
2
3
2024";

    #[test]
    fn test_sum_2000th_secret_number() {
        let result = sum_nth_secret_number(TEST_INPUT, 2000);
        assert_eq!(result, 37327623);
    }

    #[rstest]
    #[case(123, 1, 15887950)]
    #[case(123, 2, 16495136)]
    #[case(123, 3, 527345)]
    #[case(1, 2000, 8685429)]
    #[case(10, 2000, 4700978)]
    #[case(100, 2000, 15273692)]
    #[case(2024, 2000, 8667524)]
    fn test_get_nth_secret_number(#[case] num: u128, #[case] n: usize, #[case] expected: u128) {
        let result = get_nth_secret_number(num, n);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_max_bananas() {
        let result = get_max_bananas(TEST_MAX_BANANAS_INPUT, 2000);
        assert_eq!(result, 23);
    }
}
