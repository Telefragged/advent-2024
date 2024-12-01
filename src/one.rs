use std::{collections::BTreeMap, error::Error, io::{stdin, BufRead, BufReader, Read}, str::SplitWhitespace};

fn parse<R: Read>(reader: R) -> Result<(Vec<u64>, Vec<u64>), Box<dyn Error>> {
    let reader = BufReader::new(reader);

    let mut first = vec![];
    let mut second = vec![];

    fn parse_words<'a>(s: &mut SplitWhitespace<'a>) -> Option<(&'a str, &'a str)> {
        let r = (s.next()?, s.next()?);
        debug_assert_eq!(s.next(), None);
        Some(r)
    }

    for line in reader.lines() {
        let line = line?;
        let (one, two) = parse_words(&mut line.split_whitespace())
            .ok_or_else(|| format!("Cannot split {} into two parts", line))?;

        first.push(one.parse()?);
        second.push(two.parse()?);
    }

    Ok((first, second))
}

fn difference(first: &[u64], second: &[u64]) -> u64 {
    first
        .iter()
        .zip(second)
        .map(|(one, two)| one.abs_diff(*two))
        .sum()
}

fn similarity(first: &[u64], second: &[u64]) -> u64 {
    let mut second_counts = BTreeMap::<u64, u64>::new();

    for id in second {
        *second_counts.entry(*id).or_default() += 1;
    }

    first
        .iter()
        .map(|id| id * second_counts.get(id).copied().unwrap_or_default())
        .sum()
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let (mut first, mut second) = parse(stdin())?;
    first.sort_unstable();
    second.sort_unstable();
    let difference = difference(&first, &second);
    let similarity = similarity(&first, &second);

    println!("difference: {}", difference);
    println!("similarity: {}", similarity);

    Ok(())
}
