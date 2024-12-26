use std::collections::{HashMap, HashSet};
use std::fs;

pub fn part1() -> Result<(), Box<dyn std::error::Error>>{
    let path = "inputs/day5/part1.txt";
    let data = fs::read_to_string(&path)?;

    let split = data.split("\n\n").collect::<Vec<&str>>();
    let rules_string = split[0];
    let pages_string = split[1];

    let rules = rules_string.trim()
        .split("\n")
        .map(|x| x.split("|"))
        .map(|x| x.collect::<Vec<&str>>())
        .map(|x| (
            x[0].parse::<i32>().expect("couldn't parse int"),
            x[1].parse::<i32>().expect("couldn't parse int")
        )).collect::<Vec<(i32, i32)>>();

    let pages = pages_string.trim()
        .split("\n")
        .map(|x| x.split(","))
        .map(|x| x.map(|e| e.parse::<i32>().expect("couldn't parse int")))
        .map(|x| x.collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let mut before: HashMap<i32, HashSet<i32>>  = HashMap::new();
    let mut after: HashMap<i32, HashSet<i32>>  = HashMap::new();

    for (b, a) in &rules {
        let maybe = before.get_mut(b);
        maybe.map(|x| x.insert(a.clone()))
            .or_else(|| {
                let mut set = HashSet::new();
                set.insert(a.clone());
                before.insert(b.clone(), set);
                None
            });

        let maybe = after.get_mut(a);
        maybe.map(|x| x.insert(b.clone()))
            .or_else(|| {
                let mut set = HashSet::new();
                set.insert(b.clone());
                after.insert(a.clone(), set);
                None
            });
    }

    let mut sum = 0;
    for page in pages {
        let mut pass = true;
        for i in 0..page.len() {
            if !pass {
                break;
            }

            let or_else = &HashSet::new();
            let local_before = before.get(&page[i]).or_else(|| Some(or_else)).unwrap();
            let local_after = after.get(&page[i]).or_else(|| Some(or_else)).unwrap();

            for j in 0..i {
                if !local_after.contains(&page[j]) {
                    pass = false;
                    break;
                }
            }

            for j in i + 1..page.len() {
                if !local_before.contains(&page[j]) {
                    pass = false;
                    break;
                }
            }
        }

        if pass {
            let mid = page[page.len() / 2];
            sum += mid;
            println!("passes {:?}, mid: {}", page, mid);
        }
    }

    println!("sum: {}", sum);

    Ok(())
}