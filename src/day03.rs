use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut gamma_rate: usize = 0;
    let mut epsilon_rate: usize = 0;
    let mut ones_map: HashMap<usize, usize> = HashMap::new(); // position, count of ones
    let line_count = lines.len();
    ones_map = get_ones_map(&lines);
    println!("{:?}", ones_map);
    for key in ones_map.keys().sorted().rev() {
        if ones_map[key] > line_count / 2 {
            // one was the most common for the position
            gamma_rate += 2usize.pow(*key as u32);
        } else {
            // zero was the most common for the position
            // println!("{}", 2usize.pow(*key as u32));
            epsilon_rate += 2usize.pow(*key as u32);
        }
    }
    println!("ep:{}", epsilon_rate);
    println!("gamma: {}", gamma_rate);
    let result = epsilon_rate * gamma_rate;
    println!("{:?}", &result);
}

pub fn part2(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>();
    let mut o2_lines = lines.clone();
    let mut co2_lines = lines.clone();
    let line_length = lines[0].len();
    println!("{:?}", o2_lines);
    let mut process_o2 = true;
    let mut process_co2 = true;
    for bit in (0..line_length).into_iter() {
        if o2_lines.len() == 1 {
            process_o2 = false;
        }
        if process_o2 {
            let o2_map = get_ones_and_zero_map(&o2_lines);
            let o2_bit_map = o2_map[&bit];
            if o2_bit_map.0 == o2_bit_map.1 || o2_bit_map.1 > o2_bit_map.0 {
                o2_lines.retain(|x| {
                    x.chars()
                        .into_iter()
                        .nth(bit)
                        .expect("Failed to get nth character")
                        == '1'
                })
            } else {
                o2_lines.retain(|x| {
                    x.chars()
                        .into_iter()
                        .nth(bit)
                        .expect("Failed to get nth character")
                        == '0'
                })
            }
        }
    }
    for bit in (0..line_length).into_iter() {
        if co2_lines.len() == 1 {
            process_co2 = false;
        }
        if process_co2 {
            let co2_map = get_ones_and_zero_map(&co2_lines);
            let co2_bit_map = co2_map[&bit];
            if co2_bit_map.0 == co2_bit_map.1 || co2_bit_map.1 > co2_bit_map.0 {
                co2_lines.retain(|x| {
                    x.chars()
                        .into_iter()
                        .nth(bit)
                        .expect("Failed to get nth character")
                        == '0'
                })
            } else {
                co2_lines.retain(|x| {
                    x.chars()
                        .into_iter()
                        .nth(bit)
                        .expect("Failed to get nth character")
                        == '1'
                })
            }
        }
    }
    println!("o2 {:?} and co2 {:?}", o2_lines, co2_lines);
    let o2_rating: usize = usize::from_str_radix(o2_lines[0], 2).expect("Failed to parse binary to int");
    let co2_rating: usize = usize::from_str_radix(co2_lines[0], 2).expect("Failed to parse binary to int");
    let result = o2_rating * co2_rating;
    println!("{}", result);
}

fn get_ones_map(input: &Vec<&str>) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();
    for line in input {
        for (position, character) in line.chars().enumerate() {
            match character {
                '1' => {
                    let entry = map.entry(position).or_insert(0);
                    *entry += 1;
                }
                _ => {}
            }
        }
    }
    map
}

fn get_ones_and_zero_map(input: &Vec<&str>) -> HashMap<usize, (usize, usize)> {
    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();
    for line in input {
        for (position, character) in line.chars().enumerate() {
            match character {
                '0' => {
                    let entry = map.entry(position).or_insert((0, 0));
                    entry.0 += 1;
                }
                '1' => {
                    let entry = map.entry(position).or_insert((0, 0));
                    entry.1 += 1;
                }
                _ => {}
            }
        }
    }
    map
}
