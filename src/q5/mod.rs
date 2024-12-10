use constraint::Constraint;

mod constraint;

pub fn get_total_valid_middle_page_numbers(input: &str) -> usize {
    let middle_page_numbers = get_valid_middle_page_numbers(input);
    middle_page_numbers.iter().sum()
}

pub fn get_total_invalid_middle_page_numbers(input: &str) -> usize {
    let middle_page_numbers = get_invalid_middle_page_numbers(input);
    middle_page_numbers.iter().sum()
}

fn get_valid_middle_page_numbers(input: &str) -> Vec<usize> {
    let (constraints, updates) = parse_input(input);
    let mut middle_page_numbers = Vec::new();

    for update in updates {
        if is_update_valid(&constraints, &update) {
            let middle_page_number = update[update.len() / 2];
            middle_page_numbers.push(middle_page_number);
        }
    }

    middle_page_numbers
}

fn get_invalid_middle_page_numbers(input: &str) -> Vec<usize> {
    let (constraints, updates) = parse_input(input);
    let mut middle_page_numbers = Vec::new();

    for update in updates {
        if !is_update_valid(&constraints, &update) {
            let fixed_update = fix_update(&constraints, &update);
            let middle_page_number = fixed_update[fixed_update.len() / 2];
            middle_page_numbers.push(middle_page_number);
        }
    }

    middle_page_numbers
}

fn parse_input(input: &str) -> (Vec<Constraint>, Vec<Vec<usize>>) {
    let mut constraints = Vec::new();
    let mut updates = Vec::new();

    let mut constraints_section = true;

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            constraints_section = false;
            continue;
        }

        if constraints_section {
            let constraint = line.parse().unwrap();
            constraints.push(constraint);
        } else {
            let values = line
                .split_terminator(',')
                .map(|v| v.parse().unwrap())
                .collect();
            updates.push(values);
        }
    }

    (constraints, updates)
}

fn is_update_valid(constraints: &[Constraint], update: &[usize]) -> bool {
    for constraint in constraints {
        if !constraint.is_valid(update) {
            return false;
        }
    }
    true
}

fn fix_update(constraints: &[Constraint], update: &[usize]) -> Vec<usize> {
    let mut fixed_update = update.to_vec();

    let relevant_constraints = constraints
        .iter()
        .filter(|c| c.is_relevant(update))
        .collect::<Vec<_>>();

    fixed_update.sort_by(|a, b| {
        use std::cmp::Ordering;
        let relevant_constraint = relevant_constraints
            .iter()
            .find(|c| (c.x == *a && c.y == *b) || (c.x == *b && c.y == *a));
        match relevant_constraint {
            Some(constraint) => {
                if constraint.x == *a {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            None => Ordering::Equal,
        }
    });
    fixed_update
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::redundant_static_lifetimes)]
    const EXAMPLE_INPUT: &'static str = "47|53
                                         97|13
                                         97|61
                                         97|47
                                         75|29
                                         61|13
                                         75|53
                                         29|13
                                         97|29
                                         53|29
                                         61|53
                                         97|53
                                         61|29
                                         47|13
                                         75|47
                                         97|75
                                         47|61
                                         75|61
                                         47|29
                                         75|13
                                         53|13
             
                                         75,47,61,53,29
                                         97,61,53,29,13
                                         75,29,13
                                         75,97,47,61,53
                                         61,13,29
                                         97,13,75,29,47";
    #[test]
    fn test_get_total_valid_middle_page_numbers() {
        let result = get_total_valid_middle_page_numbers(EXAMPLE_INPUT);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_get_total_invalid_middle_page_numbers() {
        let result = get_total_invalid_middle_page_numbers(EXAMPLE_INPUT);
        assert_eq!(result, 123);
    }
}
