use anyhow::Result;
use log::{debug, info};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub async fn solve() -> Result<()> {
    let file = File::open("src/days/1/resources/input.txt")?;
    let reader = BufReader::new(file);

    let numbers: Vec<i64> = reader
        .lines()
        .map(|line| {
            let string = line.expect("Could not parse line!");
            string
                .parse::<i64>()
                .expect("Could not convert string to i32!")
        })
        .collect();

    info!("Part 1 Answer is: {}", part_one(2020, &numbers.clone())?);
    info!("Part 2 Answer is: {}", part_two(2020, numbers.clone())?);

    Ok(())
}

pub fn part_one(target: i64, numbers: &[i64]) -> Result<i64> {
    for number in numbers {
        let search_for = target - number;
        if numbers.contains(&search_for) {
            debug!("{} and {} sum to {}", number, search_for, target);
            return Ok(search_for * number);
        }
    }

    Err(anyhow::anyhow!(
        "Could not find 2 entries that sum to {}",
        target
    ))
}

fn part_two(target: i64, numbers: Vec<i64>) -> Result<i64> {
    for i in 0..numbers.len() {
        let value = numbers[i];

        let result = part_one(target - value, &numbers[(i + 1)..]);

        match result {
            Ok(answer) => {
                debug!("Found 2 sum solution {}", answer);
                return Ok(answer * value);
            }
            Err(_error) => {
                debug!("Couldn't find 3 sum for index {}", i);
            }
        }
    }

    Err(anyhow::anyhow!(
        "Could not find 3 entries that sum to {}",
        target
    ))
}
