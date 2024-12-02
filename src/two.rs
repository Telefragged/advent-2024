use std::{
    error::Error,
    io::{stdin, BufRead, BufReader, Read},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Increasing,
    Decreasing,
}

impl TryFrom<std::cmp::Ordering> for Direction {
    type Error = ();

    fn try_from(value: std::cmp::Ordering) -> Result<Self, Self::Error> {
        match value {
            std::cmp::Ordering::Less => Ok(Self::Increasing),
            std::cmp::Ordering::Greater => Ok(Self::Decreasing),
            std::cmp::Ordering::Equal => Err(()),
        }
    }
}

fn parse<R: Read>(reader: R) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    let reader = BufReader::new(reader);

    reader
        .lines()
        .map(|line| {
            let line = line?;

            line.split_whitespace()
                .map(|word| word.parse().map_err(|e| Box::new(e) as _))
                .collect()
        })
        .collect()
}

fn safe_reports(reports: Vec<Vec<u64>>) -> usize {
    fn is_safe(report: Vec<u64>) -> bool {
        let mut direction = None;

        for levels in report.windows(2) {
            let first = levels[0];
            let second = levels[1];
            let difference = first.abs_diff(second);
            if !(1..=3).contains(&difference) {
                return false;
            }

            match (direction, first.cmp(&second)) {
                (Some(Direction::Decreasing), std::cmp::Ordering::Less)
                | (Some(Direction::Increasing), std::cmp::Ordering::Greater) => return false,
                (_, std::cmp::Ordering::Less) => direction = Some(Direction::Increasing),
                (_, std::cmp::Ordering::Greater) => direction = Some(Direction::Decreasing),
                (_, std::cmp::Ordering::Equal) => {
                    unreachable!("The levels have a difference in range 1..=3 and cannot be equal")
                }
            }
        }

        true
    }

    reports
        .into_iter()
        .map(is_safe)
        .filter(|safe| *safe)
        .count()
}

fn safe_reports_dampened(reports: Vec<Vec<u64>>) -> usize {
    reports
        .into_iter()
        .map(|report| {
            let skipped = (0..report.len()).any(|skip| {
                let new_report = report
                    .iter()
                    .enumerate()
                    .filter(|(n, _)| *n != skip)
                    .map(|(_, level)| *level)
                    .collect();

                safe_reports(vec![new_report]) > 0
            });

            safe_reports(vec![report]) > 0 || skipped
        })
        .filter(|is_safe| *is_safe)
        .count()
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let levels = parse(stdin())?;
    let safe_reports = safe_reports(levels.clone());
    let safe_reports_dampened = safe_reports_dampened(levels);

    println!("safe reports: {}", safe_reports);
    println!("safe reports with dampener: {}", safe_reports_dampened);

    Ok(())
}
