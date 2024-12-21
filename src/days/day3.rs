use std::fs;
use regex::Regex;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let path = "inputs/day3/part1.txt";
    let data = fs::read_to_string(&path)?;

    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)")?;

    let mut sum = 0;
    for caps in re.find_iter(&data) {
        let cleaned = caps.as_str()
            .replace("mul(", "")
            .replace(")", "");
        let result = cleaned.split(",")
            .collect::<Vec<&str>>();
        sum += result[0].parse::<i32>()? * result[1].parse::<i32>()?;
    }

    println!("Day 3, Part 1, Sum: {}", sum);

    Ok(())
}

#[derive(Debug)]
struct Element {
    mode: String,
    start_index: usize,
    data: Option<String>
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let path = "inputs/day3/part2.txt";
    let data = fs::read_to_string(&path)?;

    let mults = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)")?;
    let dos = Regex::new(r"do\(\)")?;
    let donts = Regex::new(r"don't\(\)")?;

    let mut joined_list: Vec<Element> = vec![];

    for mat in mults.find_iter(&data) {
        joined_list.push(Element {
            mode: String::from("mul"),
            start_index: mat.start(),
            data: Some(String::from(mat.as_str()))
        })
    }

    for mat in dos.find_iter(&data) {
        joined_list.push(Element {
            mode: String::from("do"),
            start_index: mat.start(),
            data: None
        })
    }

    for mat in donts.find_iter(&data) {
        joined_list.push(Element {
            mode: String::from("dont"),
            start_index: mat.start(),
            data: None
        })
    }

    joined_list.sort_by(|x, y| x.start_index.cmp(&y.start_index));


    let mut enabled = true;
    let mut sum = 0;
    for elem in joined_list {
        if elem.mode == "do" {
            enabled = true;
        } else if elem.mode == "dont" {
            enabled = false;
        } else if enabled && elem.mode == "mul" {
            let filtered = elem.data.unwrap()
                .replace("mul(", "")
                .replace(")", "");
            let split = filtered
                .split(",")
                .collect::<Vec<&str>>();

            sum += split[0].parse::<i32>()? * split[1].parse::<i32>()?;
        }
    }

    println!("Day 3, Part 2, Sum: {}", sum);

    Ok(())
}