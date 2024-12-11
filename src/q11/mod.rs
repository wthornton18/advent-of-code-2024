mod stone;
use hashbrown::HashMap;
use stone::Stone;

fn parse_input(input: &str) -> Vec<Stone> {
    input
        .split_ascii_whitespace()
        .map(|line| {
            let line = line.trim();

            line.parse().unwrap()
        })
        .collect()
}

pub fn count_total_stones(input: &str, blink: usize) -> usize {
    let mut map = HashMap::new();
    let stones = parse_input(input);
    stones
        .iter()
        .map(|s| get_stone_count(&mut map, (*s, blink)))
        .sum()
}

fn get_stone_count(
    map: &mut HashMap<(Stone, usize), usize>,
    (stone, blink): (Stone, usize),
) -> usize {
    if let Some(count) = map.get(&(stone, blink)) {
        return *count;
    }

    let ret = if blink == 0 {
        1
    } else {
        let (stone, other_stone) = stone.blink();

        let stone_count = get_stone_count(map, (stone, blink - 1));
        stone_count
            + other_stone
                .map(|other_stone| get_stone_count(map, (other_stone, blink - 1)))
                .unwrap_or(0)
    };
    map.insert((stone, blink), ret);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_count_total_stones_low_blink() {
        assert_eq!(count_total_stones(TEST_INPUT, 6), 22);
    }

    #[test]
    fn test_count_total_stones() {
        assert_eq!(count_total_stones(TEST_INPUT, 25), 55312)
    }
}
