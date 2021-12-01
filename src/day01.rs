use itertools::Itertools;

pub fn part1(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>();
    let lines = lines.as_slice();
    let mut counter: usize = 0;
    for window in lines.windows(2) {
        let first: usize = window[0].parse().expect("Failed to parse first value.");
        let second: usize = window[1].parse().expect("Failed to parse second value");
        if second > first {
            counter += 1;
        }
    }
    println!("{}", counter);
}

pub fn part2(input: String) {
    let input = input.chars().collect::<String>();
    let mut lines = input.lines().collect::<Vec<&str>>();
    let lines: Vec<usize>  = lines.iter_mut().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let mut counter: usize = 0;

    for (first, second, third, fourth) in lines.iter().tuple_windows() {
        let first_sum: usize = first + second + third;
        let second_sum: usize = second + third + fourth;
        if second_sum > first_sum {
            counter += 1;
        }
    }
    println!("{}", counter);
}