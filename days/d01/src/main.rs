//! Location IDs should probably be unsigned integers, but no such information
//! is given, so let's assume they can be negative too.

use std::{
    collections::HashMap,
    env,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{anyhow, bail, Context, Error, Result};

fn read_input_lists(path: impl AsRef<Path>) -> Result<(Vec<i32>, Vec<i32>)> {
    const BUF_SZ: usize = 1 << 21;

    let br = BufReader::with_capacity(
        BUF_SZ,
        OpenOptions::new()
            .read(true)
            .open(&path)
            .with_context(|| format!("failed to open file '{}'", path.as_ref().display()))?,
    );

    let mut l1 = Vec::with_capacity(1000);
    let mut l2 = l1.clone();
    br.lines().enumerate().try_for_each(|(i, line)| {
        let line = line.with_context(|| format!("error reading input line {i}"))?;
        let mut tokens = line.split_whitespace();
        l1.push(
            tokens
                .next()
                .with_context(|| format!("line {i} is empty"))?
                .parse()
                .with_context(|| format!("line {i}: failed to parse integer from 1st token"))?,
        );
        l2.push(
            tokens
                .next()
                .with_context(|| format!("line {i} has only 1 token"))?
                .parse()
                .with_context(|| format!("line {i}: failed to parse integer from 2nd token"))?,
        );
        debug_assert!(tokens.next().is_none());
        Ok::<(), Error>(())
    })?;
    debug_assert_eq!(l1.len(), l2.len());

    Ok((l1, l2))
}

fn part1(path: impl AsRef<Path>) -> Result<i32> {
    let (mut l1, mut l2) = read_input_lists(path).context("failed to read input lists")?;
    l1.sort_unstable();
    l2.sort_unstable();

    Ok(l1.into_iter().zip(l2).map(|(l, r)| (l - r).abs()).sum())
}

fn part2(path: impl AsRef<Path>) -> Result<i32> {
    let (l1, l2) = read_input_lists(path).context("failed to read input lists")?;

    let mut fr1 = HashMap::with_capacity(l1.len());
    let mut fr2 = fr1.clone();
    l1.into_iter().for_each(|x| *fr1.entry(x).or_insert(0) += 1);
    l2.into_iter().for_each(|x| *fr2.entry(x).or_insert(0) += 1);

    Ok(fr1
        .into_iter()
        .filter_map(|(x, f1)| fr2.get(&x).map(|&f2| x * f1 * f2))
        .sum())
}

fn main() -> Result<()> {
    let mut argv = env::args();
    let bin = argv.next().expect("argv[0] is binary's name in Unix");
    let part = argv
        .next()
        .ok_or_else(|| anyhow!("Usage: ./{bin} <PART_NO> <INPUT_FILE_PATH>"))?;
    let path = argv
        .next()
        .ok_or_else(|| anyhow!("Usage: ./{bin} <PART_NO> <INPUT_FILE_PATH>"))?;

    let result = match part.as_str() {
        "1" => part1(path),
        "2" => part2(path),
        _part => bail!("'{part}' is not a valid part number"),
    }
    .with_context(|| format!("error while running part {part} solver"))?;
    eprintln!("{result}");

    Ok(())
}
