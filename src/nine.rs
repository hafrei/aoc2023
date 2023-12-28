pub fn run(input: String) {
    let mut dataset = Vec::new();
    for l in input.lines() {
        let numbers: Vec<i32> = l
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        dataset.push(numbers);
    }

    let comp: Vec<i32> = Vec::new();
    for d in dataset {
        // for d in dataset,
        //  figure out which is higher first (cause the difference matters)
        //   w[larger] - w[smaller]
        //   if all != 0, stuff the values in a vec and do the whole thing again?
        for w in d.windows(2) {}
    }
}
