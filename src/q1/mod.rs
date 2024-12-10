use ahash::AHashMap;

use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines().collect::<Vec<_>>();

    let mut arr = Vec::with_capacity(lines.len());
    let mut other = Vec::with_capacity(lines.len());

    for line in lines {
        let line = line.trim();
        let out = line.split_whitespace().collect::<Vec<_>>();
        if out.len() != 2 {
            continue;
        }

        let a = out[0].parse().unwrap();

        let b = out[1].parse().unwrap();

        arr.push(a);
        other.push(b);
    }

    (arr, other)
}

fn compute_similarity_scores(arr: &[i32], other: &[i32]) -> Vec<i32> {
    let other = Vec::from(other);
    let unique_elements = other.iter().unique();
    let mut count = AHashMap::from_iter(unique_elements.zip(std::iter::repeat(0)));
    for elem in other.iter() {
        *count.get_mut(&elem).unwrap() += 1;
    }

    let mut similarity_scores = Vec::with_capacity(arr.len());

    for elem in arr {
        let similarity_score = count.get(elem).unwrap_or(&0) * elem;
        similarity_scores.push(similarity_score);
    }

    similarity_scores
}

fn compute_distances(arr: &mut [i32], other: &mut [i32]) -> Vec<i32> {
    arr.sort();
    other.sort();

    arr.iter()
        .zip(other.iter())
        .map(|(a, b)| (b - a).abs())
        .collect()
}

pub fn compute_total_distance(input: &str) -> i32 {
    let (mut arr, mut other) = parse_input(input);

    let result = compute_distances(&mut arr, &mut other);

    result.iter().sum()
}

pub fn compute_total_similarity_score(input: &str) -> i32 {
    let (arr, other) = parse_input(input);

    let result = compute_similarity_scores(&arr, &other);

    result.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_distances() {
        let mut arr = [3, 4, 2, 1, 3, 3];
        let mut other = [4, 3, 5, 3, 9, 3];

        let result = compute_distances(&mut arr, &mut other);

        assert_eq!(result, [2, 1, 0, 1, 2, 5]);
    }

    #[test]
    fn test_compute_total_distance() {
        let input = "3   4
                           4   3
                           2   5
                           1   3
                           3   9
                           3   3";

        assert_eq!(compute_total_distance(input), 11);
    }
    #[test]
    fn test_compute_similarity_scores() {
        let arr = [3, 4, 2, 1, 3, 3];
        let other = [4, 3, 5, 3, 9, 3];

        let result = compute_similarity_scores(&arr, &other);

        assert_eq!(result, [9, 4, 0, 0, 9, 9]);
    }

    #[test]
    fn test_compute_total_similarity_score() {
        let input = "3   4
                           4   3
                           2   5
                           1   3
                           3   9
                           3   3";

        assert_eq!(compute_total_similarity_score(input), 31);
    }
}
