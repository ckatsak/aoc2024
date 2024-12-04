#![feature(array_windows)] // part1
#![feature(iter_map_windows, iter_collect_into)] // part2

use std::{
    env,
    fs::OpenOptions,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{anyhow, bail, Context, Error, Result};

const BUF_SZ: usize = 1 << 21;

const MAX_NUM_REPORTS: usize = 1000;

const PROBLEM_DAMPENER_PERMITS: u8 = 1;

fn read_grid(path: impl AsRef<Path>) -> Result<Vec<Vec<i32>>> {
    BufReader::with_capacity(
        BUF_SZ,
        OpenOptions::new()
            .read(true)
            .open(&path)
            .with_context(|| format!("failed to open file '{}'", path.as_ref().display()))?,
    )
    .lines()
    .enumerate()
    .try_fold(
        Vec::with_capacity(MAX_NUM_REPORTS),
        |mut reports, (i, line)| {
            reports.push(
                line.with_context(|| format!("failed to read line {i}"))?
                    .split_whitespace()
                    .map(|level| {
                        level
                            .parse()
                            .with_context(|| format!("failed to parse integer from {level:?}"))
                    })
                    .collect::<Result<_>>()
                    .with_context(|| format!("error parsing line {i}"))?,
            );
            Ok::<_, Error>(reports)
        },
    )
}

fn part1(path: impl AsRef<Path>) -> Result<usize> {
    Ok(read_grid(path)
        .context("failed to read input")?
        .into_iter()
        .filter(|report| match report.len() {
            0 | 1 => true, // reports with 0 or 1 levels are trivially safe?
            _ => report.array_windows().all(|&[a, b]| {
                a.cmp(&b)
                    .eq(&report[0].cmp(&report[1])) // SAFETY: already checked len
                    .then(|| (1..=3).contains(&(a - b).abs()))
                    .unwrap_or(false)
            }),
        })
        .count())
}

fn part2(path: impl AsRef<Path>) -> Result<usize> {
    Ok(read_grid(path)
        .context("failed to read input")?
        .into_iter()
        .enumerate()
        .filter(|(_ri, report)| match report.len() {
            0 | 1 => true, // reports with 0 or 1 levels are trivially safe?
            //_ => {
            //    let check = |x: i32, y: i32| {
            //        x.cmp(&y)
            //            .eq(&report[0].cmp(&report[1])) // SAFETY: already checked len
            //            .then(|| (1..=3).contains(&(x - y).abs()))
            //            .unwrap_or(false)
            //    };
            //    let mut problem_dampener_permits = PROBLEM_DAMPENER_PERMITS;
            //    let cnt = report
            //        .array_windows()
            //        .filter(|&[a, b, c]| {
            //            eprint!("[{a}, {b}, {c}], ");
            //            match check(*a, *b) {
            //                true => true,
            //                false if problem_dampener_permits == 0 => false,
            //                false => check(*a, *c)
            //                    .then(|| {
            //                        problem_dampener_permits -= 1;
            //                        true
            //                    })
            //                    .unwrap_or(false),
            //            }
            //        })
            //        .count();
            //    eprintln!("line {i}: {cnt}/{}", report.len());
            //    // TODO
            //    cnt >= report.len() - 2
            //}
            //
            //n => {
            //    let mut problem_dampener_permits = PROBLEM_DAMPENER_PERMITS;
            //    let mut diffs = Vec::with_capacity(n - 1);
            //    let diffs = report
            //        .iter()
            //        .map_windows(|&[a, b]| a - b)
            //        .collect_into(&mut diffs);
            //    for (i, &d) in diffs[..diffs.len() - 1].iter().enumerate() {
            //        if !(1..=3).contains(&d.abs()) {
            //            if problem_dampener_permits == 0 {
            //                return false;
            //            }
            //            if !(1..=3).contains(&(d + diffs[i + 1]).abs()) {
            //                return false;
            //            }
            //            problem_dampener_permits -= 1;
            //        }
            //    }
            //    true
            //}
            //
            n => {
                let mut diffs = Vec::with_capacity(n - 1);
                let diffs = report
                    .iter()
                    .map_windows(|&[a, b]| b - a)
                    .collect_into(&mut diffs);
                let num_pos = diffs.iter().filter(|&d| (1..=3).contains(d)).count();
                let num_neg = diffs.iter().filter(|&d| (-3..=-1).contains(d)).count();
                eprint!("\n - report: {report:?};\tdiffs: {diffs:?},\tnum_pos = {num_pos}, num_neg = {num_neg} --> ");
                if num_pos >= n - 1 || num_neg >= n - 1 {
                    eprintln!("safe");
                    return true;
                }
                //if num_pos < n - 3 && num_neg < n - 3 {
                if num_pos <= n - 3 && num_neg <= n - 3 {
                    eprintln!("unsafe");
                    return false;
                }
                //assert!(num_pos == n - 3 || num_neg == n - 3);
                // TODO
                eprintln!("let's see ({_ri})");
                let diff_sign = if num_pos > num_neg { 1 } else { -1 };
                // first:
                if diffs[0].signum() != diff_sign {
                    let ret =  (1..=3).contains(&(diffs[0] + diffs[1]).abs());
                    eprintln!("first --> {}", if ret {"safe"} else {"unsafe"});
                    return ret;
                }
                // last:
                if diffs[n - 2].signum() != diff_sign {
                    let ret = (1..=3).contains(&(diffs[n - 2] + diffs[n - 3]).abs());
                    eprintln!("last --> {}", if ret {"safe"} else {"unsafe"});
                    return ret;
                }
                eprintln!("middle...");
                let ret = diffs
                    .iter()
                    .map_windows(|&[a, b, c]| {
                        if b.signum() != diff_sign {
                            // TODO
                            if (1..=3).contains(&(a + b).abs()) || (1..=3).contains(&(b + c).abs())
                            {
                                eprintln!("(safe)");
                                return true;
                            }
                        }
                        // TODO
                        false
                    })
                    .any(|x| x);
                eprintln!("middle(ret) = {}", if ret { "safe!"} else {"UNSAFE!"});
                ret
            }
        })
        .count())
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
