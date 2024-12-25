mod keylock;
use keylock::KeyLock;

fn parse_input(input: &str) -> (Vec<KeyLock>, Vec<KeyLock>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let lines = input.lines().collect::<Vec<_>>();

    let mut i = 0;

    while i < lines.len() {
        let mut j = i;
        while j < lines.len() && !lines[j].is_empty() {
            j += 1;
        }
        let mut s = String::new();

        for line in &lines[i..j] {
            s.push_str(line);
            s.push('\n');
        }

        let key_lock: KeyLock = s.parse().unwrap();

        if key_lock.is_key() {
            keys.push(key_lock);
        } else {
            locks.push(key_lock);
        }

        i = j + 1;
    }

    (keys, locks)
}

pub fn count_fitting_key_locks(input: &str) -> usize {
    let (keys, locks) = parse_input(input);

    let mut count = 0;

    for key in &keys {
        for lock in &locks {
            if key.fits(*lock) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_count_fitting_key_locks() {
        let result = count_fitting_key_locks(TEST_INPUT);
        assert_eq!(result, 3);
    }
}
