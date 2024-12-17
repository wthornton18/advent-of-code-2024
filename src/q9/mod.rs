mod disk;
use disk::{Checksum, Disk, SimpleDisk};
use std::str::FromStr;

fn pretty_print_disk(input: &[Option<usize>]) {
    for elem in input {
        match elem {
            Some(id) => print!("{}", id),
            None => print!("."),
        }
    }
    println!();
}

pub fn get_maximally_compact_checksum(input: &str) -> usize {
    let mut disk: SimpleDisk = Disk::from_str(input).unwrap().into();
    disk.maximally_compact();
    disk.checksum()
}

pub fn get_compact_no_frag_checksum(input: &str) -> usize {
    let mut disk = Disk::from_str(input).unwrap();
    disk.compact_no_frag();
    disk.checksum()
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("12345", 60)]
    #[case("2333133121414131402", 1928)]
    fn test_get_maximally_compact_checksum(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(get_maximally_compact_checksum(input), expected);
    }

    #[rstest]
    #[case("2333133121414131402", 2858)]
    fn test_get_compact_no_frag_checksum(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(get_compact_no_frag_checksum(input), expected);
    }
}
