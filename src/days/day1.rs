use std::collections::HashMap;
use std::fs;

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let path = "inputs/day1/part2.txt";
    let data = fs::read_to_string(&path)?;

    let mut left: Vec<i32> = vec![];
    let mut right = HashMap::new();

    data.split("\n")
        .map(|x| x.split(" "))
        .map(|x| x.filter(|y| y.trim() != "").collect::<Vec<&str>>())
        .for_each(|x| {
            left.push(x[0].parse::<i32>().expect("couldn't parse to int"));
            // getting the number on the right maybe
            let x1 = x[1].parse::<i32>().expect("couldn't parse to int");
            let maybe_element = right.get(&x1);
            match maybe_element {
                Some(value) => right.insert(x1, value + 1),
                None => right.insert(x1, 1)
            };
        });

    let mut similarity = 0;

    left.iter().for_each(|x| {
        if let Some(value) = right.get(x) {
            similarity += value * x;
        }
    });

    println!("Day 1, Part 2, Similarity: {}", similarity);

    Ok(())
}
