fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().expect("Invalid number"))
                .collect()
        })
        .collect()
}

#[inline(always)]
fn is_report_safe(report: &[i32]) -> bool {
    let deltas = report.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let all_positive = deltas.iter().all(|&d| d > 0);
    let all_negative = deltas.iter().all(|&d| d < 0);

    if !all_positive && !all_negative {
        return false;
    }

    deltas.iter().all(|d| d.abs() < 4)
}

pub fn count_safe_reports_with_removal(input: &str) -> usize {
    let reports = parse_input(input);
    let safeties = get_reports_safeties(&reports);

    let mut safe_count = safeties.iter().filter(|&&s| s).count();

    for (_, report) in reports.iter().enumerate().filter(|(i, _)| !safeties[*i]) {
        let mut safe = false;

        for i in 0..report.len() {
            let mut report = report.clone();
            report.remove(i);

            if is_report_safe(&report) {
                safe = true;
                break;
            }
        }

        if safe {
            safe_count += 1;
        }
    }

    safe_count
}

fn get_reports_safeties(reports: &[Vec<i32>]) -> Vec<bool> {
    reports.iter().map(|r| is_report_safe(r)).collect()
}

pub fn count_safe_reports(input: &str) -> usize {
    let reports = parse_input(input);
    let safeties = get_reports_safeties(&reports);

    safeties.iter().filter(|&&s| s).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(
        vec![7_i32, 6, 4, 2, 1], true
    )]
    #[case(
        vec![1_i32, 2, 7, 8, 9], false
    )]
    fn test_is_report_safe(#[case] report: Vec<i32>, #[case] is_safe: bool) {
        assert_eq!(is_report_safe(&report), is_safe);
    }
}
