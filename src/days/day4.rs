use std::fs;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let path = "inputs/day4/part1.txt";
    let data = fs::read_to_string(&path)?;

    let parsed = data.split("\n")
        .map(|x| x.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    // left to right, right to left, top to down, down to top
    // top right to bottom left, top left to bottom right, bottom right to top left, bottom left to top right

    let mut amount = 0;
    for i in 0..parsed.len() {
        for j in 0..parsed[i].len() {
            let column_has_size = j + 4 <= parsed[i].len();
            if column_has_size {
                // left to right and right to left
                let joined = parsed[i][j..j + 4].join("");
                if joined == "XMAS" || joined == "SAMX" {
                    amount += 1;
                }
            }

            let row_has_size = i + 4 <= parsed.len();
            if row_has_size {
                // top down
                if parsed[i][j] == "X" && parsed[i + 1][j] == "M" && parsed[i + 2][j] == "A" && parsed[i + 3][j] == "S" {
                    amount += 1;

                }
                // down to top
                else if parsed[i][j] == "S" && parsed[i + 1][j] == "A" && parsed[i + 2][j] == "M" && parsed[i + 3][j] == "X" {
                    amount += 1;
                }
            }

            if column_has_size && row_has_size {
                // top left to bottom right
                if parsed[i][j] == "X" && parsed[i + 1][j + 1] == "M" && parsed[i + 2][j + 2] == "A" && parsed[i + 3][j + 3] == "S" {
                    amount += 1;
                }
                // bottom right to top left
                else if parsed[i][j] == "S" && parsed[i + 1][j + 1] == "A" && parsed[i + 2][j + 2] == "M" && parsed[i + 3][j + 3] == "X" {
                    amount += 1;
                }

                // top right to bottom left
                if parsed[i][j + 3] == "X" && parsed[i + 1][j + 2] == "M" && parsed[i + 2][j + 1] == "A" && parsed[i + 3][j] == "S" {
                    amount += 1;
                }
                // bottom left to top right
                else if parsed[i][j + 3] == "S" && parsed[i + 1][j + 2] == "A" && parsed[i + 2][j + 1] == "M" && parsed[i + 3][j] == "X" {
                    amount += 1;
                }
            }
        }
    }

    println!("Day 4, Part 1: Amount: {}", amount);

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let path = "inputs/day4/part2.txt";
    let data = fs::read_to_string(&path)?;

    let parsed = data.split("\n")
        .map(|x| x.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut amount = 0;
    for i in 0..parsed.len() - 2 {
        for j in 0..parsed[i].len() - 2 {
            if parsed[i][j] == "M" && parsed[i][j + 2] == "S" && parsed[i + 1][j + 1] == "A" && parsed[i + 2][j] == "M" && parsed[i + 2][j + 2] == "S" {
                amount += 1;
            } else if parsed[i][j] == "S" && parsed[i][j + 2] == "M" && parsed[i + 1][j + 1] == "A" && parsed[i + 2][j] == "S" && parsed[i + 2][j + 2] == "M" {
                amount += 1;
            } else if parsed[i][j] == "M" && parsed[i][j + 2] == "M" && parsed[i + 1][j + 1] == "A" && parsed[i + 2][j] == "S" && parsed[i + 2][j + 2] == "S" {
                amount += 1;
            } else if parsed[i][j] == "S" && parsed[i][j + 2] == "S" && parsed[i + 1][j + 1] == "A" && parsed[i + 2][j] == "M" && parsed[i + 2][j + 2] == "M" {
                amount += 1;
            }
        }
    }

    println!("Day 4, Part 2, Amount: {}", amount);

    Ok(())
}