pub fn run(input: String) {
    let stripped = part_one(input.clone());
    println!("Part 1: {:?}", stripped);
    let stripped_again = part_two(input);
    println!("Part 2: {:?}", stripped_again);
}

fn part_two(mut input: String) -> u32 {
    let numbers = [
        ("oneight", "18"),
        ("twone", "21"),
        ("eightwo", "82"),
        ("eighthree", "83"),
        ("fiveight", "58"),
        ("nineight", "98"),
        ("sevenine", "79"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    for (str_n, n) in numbers {
        input = input.replace(str_n, n);
    }
    part_one(input)
}

fn part_one(input: String) -> u32 {
    input
        .split('\n')
        .map(|x| x.chars().filter(|x| x.is_ascii_digit()).collect::<String>())
        .filter(|x| !x.is_empty())
        .map(|mut x| {
            if x.len() == 1 {
                let double = x.pop().unwrap();
                x.push(double);
                x.push(double);
                x
            } else if x.len() > 2 {
                let last = x.pop().unwrap();
                while x.len() != 1 {
                    x.pop();
                }
                x.push(last);
                x
            } else {
                x
            }
        })
        .map(|x| x.parse::<u32>())
        .map(Result::unwrap)
        .sum()
}
