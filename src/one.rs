pub fn run(input: String) {
    let stripped = part_one(input.clone());
    println!("Part 1: {:?}", stripped);
    let stripped_again = part_two(input);
    println!("{:?}", stripped_again);
}

//This doesn't handle the mashup cases, I did those by hand for my second star.
// Cause I got tired of being stuck on day one for so long.
// TODO: handle it properly!
// Eg oneight == 18, not 1ight
fn part_two(input: String) -> u32 {
    let numbers = [
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

    let mut bup = input.clone();
    let mut skip = 0;
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(5)
        .for_each(|mw| {
            if skip != 0 {
                skip -=1;
            } else {
            let working = mw.iter().clone().to_owned().collect::<String>();
            for (str_n, n) in numbers {
                if working.contains(str_n) {
                    bup = bup.replacen(str_n, n, 1);
                    skip = str_n.len() -1;
                }
            }}
        });
    part_one(bup)
}

fn part_one(input: String) -> u32 {
    input
        .split('\n')
        .map(|x| x.chars().filter(|x| x.is_digit(10)).collect::<String>())
        .filter(|x| !x.is_empty())
        .map(|mut x| {
            if x.len() == 1 {
                let double = x.pop().unwrap();
                x.push(double.clone());
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
