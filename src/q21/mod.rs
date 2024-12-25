fn parse_input(input: &str) -> Vec<[u32; 3]> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let nums = line.strip_suffix("A")?;
            let mut iter = nums.chars().filter_map(|c| c.to_digit(10));
            let mut arr = [0; 3];
            arr[0] = iter.next()?;
            arr[1] = iter.next()?;
            arr[2] = iter.next()?;
            Some(arr)
        })
        .collect()
}
