use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

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

    let starting_points = get_starting_points(&chart);
    let more_steps = follow_part_two(&mut directions, &chart, starting_points);
    println!("The ghost path took {more_steps} steps");
}

fn get_starting_points(chart: &HashMap<String, (String, String)>) -> Vec<String> {
    let mut coll = Vec::new();
    for key in chart.keys() {
        if key.chars().last() == Some('A') {
            coll.push(key.to_string());
        }
    }
    coll
}

fn follow_part_two(
    directions: &mut VecDeque<Turn>,
    chart: &HashMap<String, (String, String)>,
    starts: Vec<String>,
) -> u64 {
    let mut steps: u64 = 27605384352;
    let mut active_keys = vec![
        "QNP".into(),
        "TVC".into(),
        "PDR".into(),
        "VTP".into(),
        "BNX".into(),
        "DVT".into(),
    ];

    let twenty_minutes = Duration::from_secs(300);
    let mut bookmark = Instant::now();

    println!("For the sake of your sanity...");
    for s in directions.make_contiguous().iter().cycle() {
        if bookmark.elapsed().as_secs() >= twenty_minutes.as_secs() {
            println!("{active_keys:?}\n   steps: {steps}\n");
            bookmark = Instant::now();
        }
        if active_keys
            .iter()
            .all(|k: &String| k.chars().last() == Some('Z'))
        {
            break;
        }
        let mut next_keys = Vec::new();
        for key in active_keys {
            let next_key = chart.get(&key).unwrap();
            if *s == Turn::Right {
                next_keys.push(next_key.1.clone());
            } else {
                next_keys.push(next_key.0.clone());
            }
        }
        steps += 1;
        active_keys = next_keys;
    }
    steps
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
