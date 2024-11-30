pub fn run(input: String) {
    let (times, distance) = parse_part_one_input(input.clone());
    let part_one = determine_totals(times, distance);
    println!("Part one: {}", part_one);

    let (times, distance) = parse_part_two_input(input);
    let part_two = determine_totals(times, distance);
    println!("Part two: {}", part_two);
}

fn determine_totals(times: Vec<u64>, distance: Vec<u64>) -> u64 {
    let mut totals: Vec<u64> = Vec::new();
    times.iter().zip(distance.iter()).for_each(|(t, d)| {
        let mut winners = Vec::new();
        for i in 0..*t {
            let total_distance = i * (t - i);
            if total_distance > *d && total_distance != *d {
                winners.push(i);
            }
        }
        totals.push(winners.len() as u64);
    });
    totals.iter().product::<u64>()
}

fn parse_part_two_input(input: String) -> (Vec<u64>, Vec<u64>) {
    let mut times = Vec::new();
    let mut distances = Vec::new();
    for (e, l) in input.lines().enumerate() {
        let Some((_, nums)) = l.split_once(':') else {
            unreachable!();
        };
        if e == 0 {
            let bup = nums
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<String>();
            times.push(bup.parse::<u64>().unwrap());
        } else {
            let bup = nums
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<String>();
            distances.push(bup.parse::<u64>().unwrap());
        }
    }
    (times, distances)
}

fn parse_part_one_input(input: String) -> (Vec<u64>, Vec<u64>) {
    let mut times = Vec::new();
    let mut distances = Vec::new();
    for (e, l) in input.lines().enumerate() {
        let Some((_, nums)) = l.split_once(':') else {
            unreachable!();
        };
        if e == 0 {
            times.append(
                &mut nums
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            );
        } else {
            distances.append(
                &mut nums
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            );
        }
    }
    (times, distances)
}
