use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let lines = input
        .lines()
        .map(| n |
            n.split(" ").collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}

fn part_one(lines : &Vec<Vec<&str>>) -> String {
    return lines.iter().filter(| l | {
        let range = l[0].split("-").collect::<Vec<_>>();
        let lower = range[0].parse::<usize>().unwrap();
        let upper = range[1].parse::<usize>().unwrap();
        let char = l[1].chars().collect::<Vec<_>>()[0];
        let password = l[2];

        let count = password.chars().filter(| c | *c == char).count();
        return count >= lower && count <= upper;
    }).count().to_string();
}

fn part_two(lines : &Vec<Vec<&str>>) -> String {
    return lines.iter().filter(| l | {
        let indices = l[0].split("-").collect::<Vec<_>>();
        let first = indices[0].parse::<usize>().unwrap();
        let second = indices[1].parse::<usize>().unwrap();
        let char = l[1].chars().collect::<Vec<_>>()[0];
        let password = l[2].chars().collect::<Vec<_>>();

        return (password[first - 1] == char) ^ (password[second - 1] == char);
    }).count().to_string();
}
