pub fn run(input: String) {
    let mut dataset = Vec::new();
    for l in input.lines() {
        let numbers: Vec<i32> = l
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        dataset.push(numbers);
    }

    let mut forecast = Vec::new();
    for d in dataset {
        let current_len = d.len();
        let top_forecaset = d.last().expect("Outer dataset is empty");
        let extrapolated = top_forecaset + calculate_prediction(&d, current_len); 
        forecast.push(extrapolated);
    }
    // println!("Forecast: {forecast:?}");
    let res: i32 = forecast.iter().sum();
    println!("Sum is {res}");  //1889451523 is too high
                                //682999713 is too low
}

fn calculate_prediction(consideration: &Vec<i32>, expected_len: usize) -> i32 {
    let mut direction: Option<Direction> = None;
        let mut res = Vec::with_capacity(expected_len);
        for c in consideration.windows(2) {
            if c[0] > c[1] {
                direction = Some(Direction::Decrease);
                let sub = c[0] - c[1];
                res.push(sub);
            } else {
                direction = Some(Direction::Increase);
                let sub = c[0] - c[1];
                res.push(sub);
            }
        }

    let previous_data = res.last().expect("Somehow result vec has no data");

    if res.iter().all(|x| *x == *previous_data) {
        println!("\n{res:?}");
        *previous_data
    } else {
        let extrapolated = calculate_prediction(&res, expected_len - 1);
        let combined = match direction {
            Some(Direction::Increase) => previous_data + extrapolated,
            Some(Direction::Decrease) => previous_data - extrapolated,
            None => panic!("Direction was never discerned"),
        };
        println!("\n{res:?}");
        println!("Previous: {previous_data}\nExtrapolating {extrapolated}, therefore {combined}");
        combined
    }
}

enum Direction {
    Increase,
    Decrease,
}
