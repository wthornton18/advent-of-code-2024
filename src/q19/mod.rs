use hashbrown::HashMap;

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut patterns = Vec::new();
    let mut target_towels = Vec::new();
    let mut pattern_section = true;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            pattern_section = patterns.is_empty();
            continue;
        }

        if pattern_section {
            patterns.extend(line.split(", ").map(str::to_string));
        } else {
            target_towels.push(line.to_string());
        }
    }

    (patterns, target_towels)
}

pub fn count_possible_constructable_towels(input: &str) -> usize {
    let (patterns, target_towels) = parse_input(input);

    let patterns = patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let patterns = patterns.as_slice();
    let mut memo = HashMap::new();

    target_towels
        .iter()
        .map(|target_towel| construct_towel(target_towel, patterns, &mut memo))
        .filter(|&count| count > 0)
        .count()
}

pub fn count_possible_towel_arrangements(input: &str) -> usize {
    let (patterns, target_towels) = parse_input(input);

    let patterns = patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let patterns = patterns.as_slice();
    let mut memo = HashMap::new();

    target_towels
        .iter()
        .map(|target_towel| construct_towel(target_towel, patterns, &mut memo))
        .sum()
}

pub fn construct_towel<'a>(
    target_towel: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if target_towel.is_empty() {
        return 1;
    }

    if let Some(count) = memo.get(target_towel) {
        return *count;
    }
    let count = patterns
        .iter()
        .filter_map(|pattern| {
            target_towel
                .strip_prefix(pattern)
                .map(|rest| construct_towel(rest, patterns, memo))
        })
        .sum();

    memo.insert(target_towel, count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_count_possible_constructable_towels() {
        assert_eq!(count_possible_constructable_towels(TEST_INPUT), 6);
    }
}
