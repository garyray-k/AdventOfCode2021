use itertools::Itertools;

pub fn part1(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>(); 
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    for line in lines {
        let mut split = line.split(' ');
        let (instruction, value) = (split.next().expect("Failed to parse instruction"), split.next().expect("Failed to parse value"));
        // println!("{}: {}", instruction, value);
        match instruction {
            "forward" => {
                horizontal += value.parse::<usize>().expect("Failed to parse number from value.");
            },
            "up" => {
                depth -= value.parse::<usize>().expect("Failed to parse number from value.");
            },
            "down" => {
                depth += value.parse::<usize>().expect("Failed to parse number from value.");
            },
            _ => {}
        }
    }
    let result = horizontal * depth;
    println!("{}", result);
}

enum Instruction {
    Forward,
    Up,
    Down
}

pub fn part2(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>(); 
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;
    for line in lines {
        let mut split = line.split(' ');
        let (instruction, value) = (split.next().expect("Failed to parse instruction"), split.next().expect("Failed to parse value"));
        // println!("{}: {}", instruction, value);
        match instruction {
            "forward" => {
                let value: usize = value.parse().expect("Faield to parse number from value.");
                horizontal += value;
                depth += aim * value;
            },
            "up" => {
                aim -= value.parse::<usize>().expect("Failed to parse number from value.");
            },
            "down" => {
                aim += value.parse::<usize>().expect("Failed to parse number from value.");
            },
            _ => {}
        }
    }
    let result = horizontal * depth;
    println!("{}", result);
}