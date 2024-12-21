use std::fs;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let path = "inputs/day2/part1.txt";
    let data = fs::read_to_string(&path)?;

    let result = data.split("\n")
        .map(|x| x.split(" "))
        .map(|x| x.map(|x| x.parse::<i32>().expect("couldn't parse to int")))
        .map(|x| x.collect::<Vec<i32>>())
        .map(|x| {
            let safe = check_safe(&x);

            if safe {
                1
            } else {
                0
            }
        }).reduce(|acc, curr| acc + curr);

    println!("Day 2, part 1, Safe: {}", result.expect("value is empty"));

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let path = "inputs/day2/part2.txt";
    let data = fs::read_to_string(&path)?;

    let result = data.split("\n")
        .map(|x| x.split(" "))
        .map(|x| x.map(|x| x.parse::<i32>().expect("couldn't parse to int")))
        .map(|x| x.collect::<Vec<i32>>())
        .map(|x| {
            let safe = check_safe(&x);

            if safe {
                1
            } else {
                for i in 0..x.len() {
                    let mut copy = x.clone();
                    copy.remove(i);
                    let copy_safe = check_safe(&copy);
                    if copy_safe {
                        return 1;
                    }
                }
                0
            }
        }).reduce(|acc, curr| acc + curr);

    println!("Day 2, Part 2, Safe: {}", result.expect("value is empty"));

    Ok(())
}

fn check_safe(x: &Vec<i32>) -> bool {
    let mut previous = x[0];
    let growing = x[1] > previous;
    let mut safe = true;
    for i in 1..x.len() {
        let diff = (x[i] - previous).abs();
        if diff < 1 || diff > 3 {
            safe = false;
            break;
        }
        if growing {
            if previous > x[i] {
                safe = false;
                break;
            }
        } else {
            if previous < x[i] {
                safe = false;
                break;
            }
        }
        previous = x[i];
    }
    safe
}
