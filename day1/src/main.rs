use std::fs::read_to_string;

fn main() {
    let input = read_to_string("src/input.txt").unwrap();
    let numbers = input
        .lines()
        .map(| n |
            n.parse::<i32>().unwrap()
        ).collect::<Vec<i32>>();

    println!("{}", part_one(&numbers));
    println!("{}", part_two(&numbers));
}

fn part_one(numbers : &Vec<i32>) -> String {
    for n1 in numbers {
        let filtered = numbers.iter().filter(| n |
            *n + n1 == 2020
        ).collect::<Vec<_>>();

        if filtered.is_empty() { continue; }
        else { return (*filtered.first().unwrap() * n1).to_string(); }
    }

    return String::new();
}

fn part_two(numbers : &Vec<i32>) -> String {
    for n1 in numbers {
        let filtered = numbers.iter().filter(| n |
            *n + n1 < 2020
        ).collect::<Vec<_>>();

        for n2 in filtered {
            let filtered2 = numbers.iter().filter(| n |
                *n + n1 + n2 == 2020
            ).collect::<Vec<_>>();

            if filtered2.is_empty() { continue; }
            else { return (*filtered2.first().unwrap() * n1 * n2).to_string(); }
        }
    }

    return String::new();
}
