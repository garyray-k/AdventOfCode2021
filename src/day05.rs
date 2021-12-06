
use itertools::Itertools;

pub fn part1(input: String) {
    let input = input.chars().collect::<String>();
    let lines = input.lines().collect::<Vec<&str>>();
    let points: Vec<Points> = parse_points(lines);
}

pub fn part2(input: String) {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Points {
    a: Point,
    b: Point,
}

impl Points {
    fn new(point_string: Vec<&str>) -> Points {
        let mut point_iter = point_string.iter();
        let a = point_iter.next().unwrap();
        let mut a = a.split(',');
        let ax = a.next().unwrap();
        let ax = ax.parse::<usize>().expect("Failed to parse ax");
        let ay = a.next().unwrap();
        let ay = ay.parse::<usize>().expect("Failed to parse ax");
        let a = Point { x: ax, y: ay };
        let b = point_iter.next().unwrap();
        let mut b = b.split(',');
        let bx = b.next().unwrap();
        let bx = bx.parse::<usize>().expect("Failed to parse ax");
        let by = b.next().unwrap();
        let by = by.parse::<usize>().expect("Failed to parse ax");
        let b = Point { x: bx, y: by };
        Points { a, b }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

fn parse_points(lines: Vec<&str>) -> Vec<Points> {
    let mut points: Vec<Points> = vec![];
    for line in lines {
        let ab = line.split(" -> ").collect_vec();
        points.push(Points::new(ab));
    }
    return points;
}

#[test]
fn new_points() {
    let expected = Points {
        a: Point { x: 1, y: 2 },
        b: Point { x: 3, y: 4 }
    };
    let input = vec!["1,2","3,4"];
    let actual = Points::new(input);
    assert_eq!(expected, actual);
}