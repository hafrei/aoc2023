use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Right,
}

pub fn run(input: String) {
    let Some((path, mapping)) = input.split_once("\n\n") else {
        println!("Must have had an input problem to have died so soon");
        unreachable!();
    };
    let mut directions: VecDeque<Turn> = create_directions(path);
    let chart = create_map(mapping);
    let steps = follow_part_one(&mut directions, &chart);
    println!("The path took {steps} steps");
}

fn follow_part_one(
    directions: &mut VecDeque<Turn>,
    chart: &HashMap<String, (String, String)>,
) -> u32 {
    let mut steps = 0;
    let mut active_key = "AAA";

    for s in directions.make_contiguous().iter().cycle() {
        if active_key == "ZZZ" {
            break;
        }
        let next_key = chart.get(active_key).unwrap();
        if *s == Turn::Right {
            active_key = &next_key.1;
        } else {
            active_key = &next_key.0;
        }
        steps += 1;
    }
    steps
}

fn create_directions<'a>(path: &'a str) -> VecDeque<Turn> {
    let mut directions: VecDeque<Turn> = VecDeque::new();
    for c in path.trim().chars() {
        if c.eq(&'R') {
            directions.push_back(Turn::Right);
        } else {
            directions.push_back(Turn::Left);
        }
    }
    directions
}

fn create_map<'a>(mapping: &'a str) -> HashMap<String, (String, String)> {
    let mut chart = HashMap::new();
    for m in mapping.lines() {
        let rmv = m.replace("=", "");
        let rmv = rmv.replace("(", "");
        let rmv = rmv.replace(")", "");
        let rmv = rmv.replace(",", "");
        let parsed: Vec<String> = rmv
            .trim()
            .split_whitespace()
            .map(|x| x.to_owned())
            .collect::<Vec<String>>();
        chart.insert(parsed[0].clone(), (parsed[1].clone(), parsed[2].clone()));
    }
    chart
}
