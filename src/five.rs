use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mapping {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
    Finished,
}

impl Mapping {
    pub fn determine(input: &str) -> Self {
        if input.contains("seed") & input.contains("soil") {
            Mapping::SeedToSoil
        } else if input.contains("soil") & input.contains("fertilizer") {
            Mapping::SoilToFertilizer
        } else if input.contains("fertilizer") & input.contains("water") {
            Mapping::FertilizerToWater
        } else if input.contains("water") & input.contains("light") {
            Mapping::WaterToLight
        } else if input.contains("light") & input.contains("temperature") {
            Mapping::LightToTemperature
        } else if input.contains("temperature") & input.contains("humidity") {
            Mapping::TemperatureToHumidity
        } else {
            Mapping::HumidityToLocation
        }
    }
    pub fn next(&mut self) {
        *self = match self {
            Mapping::SeedToSoil => Mapping::SoilToFertilizer,
            Mapping::SoilToFertilizer => Mapping::FertilizerToWater,
            Mapping::FertilizerToWater => Mapping::WaterToLight,
            Mapping::WaterToLight => Mapping::LightToTemperature,
            Mapping::LightToTemperature => Mapping::TemperatureToHumidity,
            Mapping::TemperatureToHumidity => Mapping::HumidityToLocation,
            Mapping::HumidityToLocation => Mapping::Finished,
            Mapping::Finished => unreachable!(),
        };
    }
}

#[derive(Clone, Copy, Debug)]
struct Page {
    mapping: Mapping,
    destination_end: u64,
    source_start: u64,
    source_end: u64,
}

impl Page {
    pub fn destination_value(&self, input: u64) -> Option<u64> {
        if input >= self.source_start && input <= self.source_end {
            let range = self.source_end - input;
            Some(self.destination_end - range)
        } else {
            None
        }
    }
}

struct Almanac {
    pages: Vec<Page>,
    seeds: Vec<u64>,
}

impl Almanac {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            seeds: Vec::new(),
        }
    }

    pub fn fill(&mut self, input: String) {
        let mut current_mapping = Mapping::SeedToSoil;
        for lin in input.lines() {
            if lin.is_empty() {
                continue;
            } else if lin.contains("seeds:") {
                let Some((_, nums)) = lin.split_once(' ') else {
                    unreachable!()
                };
                self.seeds.append(
                    &mut nums
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>(),
                );
            } else if lin.chars().all(|c| !c.is_numeric()) {
                current_mapping = Mapping::determine(lin);
            } else {
                let parsing = lin
                    .trim()
                    .split_whitespace()
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                let page = Page {
                    mapping: current_mapping,
                    destination_end: parsing[0] + parsing[2] - 1,
                    source_start: parsing[1],
                    source_end: parsing[1] + parsing[2] - 1,
                };
                self.pages.push(page);
            }
        }
    }

    fn find_lowest_location(&self, seed: u64) -> u64 {
        let mut map_to_find = Mapping::SeedToSoil;
        let mut next_dest = seed;
        while map_to_find != Mapping::Finished {
            let maybe_source: Option<u64> = self
                .pages
                .iter()
                .filter(|x| x.mapping == map_to_find)
                .filter_map(|x| x.destination_value(next_dest))
                .collect::<Vec<u64>>()
                .pop();
            if maybe_source.is_some() {
                next_dest = maybe_source.unwrap();
            }
            map_to_find.next();
        }
        next_dest
    }

    pub fn part_one(&self) -> u64 {
        let mut results = Vec::new();
        println!(
            "About to process {} seeds. Should be pretty quick.",
            self.seeds.len()
        );
        self.seeds
            .par_iter()
            .map(|x| self.find_lowest_location(*x))
            .collect_into_vec(&mut results);
        *results.iter().min().unwrap()
    }

    pub fn part_two(&self) -> u64 {
        let mut all_seeds = Vec::new();
        let mut results = Vec::new();
        for w in self.seeds.chunks(2) {
            for i in w[0]..(w[1] + w[0]) {
                all_seeds.push(i);
            }
        }
        println!(
            "\nAbout to process {} seeds. \nThis is going to take awhile, every core in your machine, and all of your ram.\nMaybe go for a run or something.",
            all_seeds.len()
        );
        all_seeds
            .par_iter()
            .map(|x| self.find_lowest_location(*x))
            .collect_into_vec(&mut results);
        *results.iter().min().unwrap()
    }
}

pub fn run(input: String) {
    let mut almanac = Almanac::new();
    almanac.fill(input);
    let lowest = almanac.part_one();
    println!("The lowest location is {lowest}");
    let actual_lowest = almanac.part_two();
    println!("The real lowest location is {actual_lowest}");
}
