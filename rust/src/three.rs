use ansiterm::Colour;
use core::fmt::{self, Display};

#[derive(Clone, Copy, Debug)]
struct Section {
    x: usize,
    y: usize,
    series: Option<usize>,
    symbol: char,
    colour: Colour,
}

impl PartialEq for Section {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y && self.x == other.x
    }
}

impl Eq for Section {}

impl Section {
    fn new(x: usize, y: usize, symbol: char, s: &mut usize) -> Self {
        let (series, colour) = match symbol {
            '.' => (None, Colour::DarkGray),
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => (None, Colour::White),
            _ => {
                let ret = (Some(*s), Colour::Green);
                *s += 1;
                ret
            }
        };
        Self {
            x,
            y,
            series,
            symbol,
            colour,
        }
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.colour.paint(self.symbol.to_string().as_str()))
    }
}

pub fn run(input: String) {
    part_one(input.clone());

    let mut width = 0;
    let mut grid = Vec::new();
    let mut series = 1;
    for (y, lin) in input.lines().enumerate() {
        if width == 0 {
            width = lin.len() - 1;
        }
        for (x, c) in lin.chars().enumerate() {
            let sec = Section::new(x, y, c, &mut series);
            grid.push(sec);
        }
    }
    let height = grid.len() / width;

    //OH NO IT'S SO SLOW
    let comp = grid.clone();
    let _ = comp
        .iter()
        .enumerate()
        .filter(|(_, x)| x.symbol == '*')
        .map(|(e, sym)| {
            let mut retry = 5; //3 is too low. Why? Haha I dunno
            let mut active = Some(e);
            while active.is_some() {
                let next = active.unwrap();
                if grid[next].colour != Colour::Green {
                    grid[next].colour = Colour::BrightPurple;
                }
                grid[next].series = sym.series;
                let x = grid[next].x;
                let y = grid[next].y;
                active = check_for_num(x, y, width, height, &grid);
                if active.is_none() {
                    retry -= 1;
                    active = Some(e);
                }
                if retry == 0 {
                    break;
                }
            }
            *sym
        })
        .collect::<Vec<Section>>();

    for x in grid.iter() {
        print!("{x}");
        if x.x == width {
            println!("");
        }
    }

    println!("");

    let mut acc = 0;
    for s in 1..series {
        let pile = grid
            .iter()
            .cloned()
            .filter(|x| x.symbol.is_ascii_digit() && x.series == Some(s))
            .collect::<Vec<Section>>();
        let mut numbers = Vec::new();
        let mut num = Vec::new();
        let mut prev_y = 999;
        let mut prev_x = 999;
        for l in pile.iter() {
            println!("{l:?}");
            if prev_y == 999 {
                prev_y = l.y;
            }

            if prev_x == 999 {
                prev_x = l.x;
            }

            if prev_y == l.y {
                // println!("same row!");
                if l.x - prev_x >= 2 {
                    if !num.is_empty() {
                        let sum = num.iter().collect::<String>().parse::<u32>().unwrap();
                        numbers.push(sum);
                        num.clear();
                    }
                    num.push(l.symbol);
                } else {
                    // println!("same number!");
                    num.push(l.symbol);
                    if num.len() == 3 {
                        let sum = num.iter().collect::<String>().parse::<u32>().unwrap();
                        numbers.push(sum);
                        num.clear();
                    }
                }
            } else {
                // println!("different row!");
                if !num.is_empty() {
                    let sum = num.iter().collect::<String>().parse::<u32>().unwrap();
                    numbers.push(sum);
                    num.clear();
                }
                num.push(l.symbol);
            }

            //update previous
            prev_y = l.y;
            prev_x = l.x;
        }
        // println!("{num:?}, {acc}");
        if !num.is_empty() {
            let sum = num.iter().collect::<String>().parse::<u32>().unwrap();
            numbers.push(sum);
            num.clear();
        }
        if numbers.len() > 1 {
            let product = numbers.iter().product::<u32>();
            println!("{} = {acc} + {product}\n", acc + product);
            acc += product;
        }
    }
    println!("{acc}");
}

fn part_one(input: String) {
    let mut width = 0;
    let mut grid = Vec::new();
    let mut series = 1;
    for (y, lin) in input.lines().enumerate() {
        if width == 0 {
            width = lin.len() - 1;
        }
        for (x, c) in lin.chars().enumerate() {
            let sec = Section::new(x, y, c, &mut series);
            grid.push(sec);
        }
    }
    let height = grid.len() / width;

    //OH NO IT'S SO SLOW
    let comp = grid.clone();
    let _ = comp
        .iter()
        .enumerate()
        .filter(|(_, x)| x.series.is_some())
        .map(|(e, sym)| {
            let mut retry = 5; //3 is too low. Why? Haha I dunno
            let mut active = Some(e);
            while active.is_some() {
                let next = active.unwrap();
                if grid[next].colour != Colour::Green {
                    grid[next].colour = Colour::BrightPurple;
                }
                grid[next].series = sym.series;
                let x = grid[next].x;
                let y = grid[next].y;
                active = check_for_num(x, y, width, height, &grid);
                if active.is_none() {
                    retry -= 1;
                    active = Some(e);
                }
                if retry == 0 {
                    break;
                }
            }
            *sym
        })
        .collect::<Vec<Section>>();

    let mut acc = 0;
    for s in 1..series {
        let pile = grid
            .iter()
            .cloned()
            .filter(|x| x.symbol.is_ascii_digit() && x.series == Some(s))
            .collect::<Vec<Section>>();
        let mut num = Vec::new();
        let mut prev_y = 999;
        let mut prev_x = 999;
        for l in pile.iter() {
            println!("{l:?}");
            if prev_y == 999 {
                prev_y = l.y;
            }

            if prev_x == 999 {
                prev_x = l.x;
            }

            if prev_y == l.y {
                // println!("same row!");
                if l.x - prev_x >= 2 {
                    if !num.is_empty() {
                        let sum = num.iter().collect::<String>().parse::<u32>().unwrap();
                        acc += sum;
                        num.clear();
                        println!("{} = {acc} + {sum}", acc + sum);
                    }
                    num.push(l.symbol);
                } else {
                    // println!("same number!");
                    num.push(l.symbol);
                    if num.len() == 3 {
                        let sum = num.iter().collect::<String>().parse::<u32>().unwrap();
                        println!("{} = {acc} + {sum}", acc + sum);
                        acc += sum;
                        num.clear();
                    }
                }
            } else {
                // println!("different row!");
                if !num.is_empty() {
                    let sum = num.iter().collect::<String>().parse::<u32>().unwrap();
                    println!("{} = {acc} + {sum}", acc + sum);
                    acc += sum;
                    num.clear();
                }
                num.push(l.symbol);
            }

            //update previous
            prev_y = l.y;
            prev_x = l.x;
        }
        // println!("{num:?}, {acc}");
        if !num.is_empty() {
            let sum = num.iter().collect::<String>().parse::<u32>().unwrap();
            println!("{} = {acc} + {sum}", acc + sum);
            acc += sum;
            num.clear();
        }
        println!("");
    }
    println!("{acc}"); //528305 is too high
}

/// -x, -y | x, -y | +x, -y
/// -------|-------|-------
/// -x,  y | x,  y | +x,  y
/// -------|-------|-------
/// -x, +y | x, +y | +x, +y

fn check_for_num(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    grid: &[Section],
) -> Option<usize> {
    let search = if x == 0 && y == 0 {
        vec![(x, y + 1), (x + 1, y), (x + 1, y + 1)]
    } else if x == width && y == height {
        vec![(x - 1, y - 1), (x - 1, y), (x, y - 1)]
    } else if x == 0 && y == height {
        vec![(x, y - 1), (x + 1, y - 1), (x + 1, y)]
    } else if x == width && y == 0 {
        vec![(x - 1, y), (x - 1, y + 1), (x, y + 1)]
    } else if x == 0 {
        vec![
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    } else if x == width {
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
        ]
    } else if y == 0 {
        vec![
            (x - 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    } else if y == height {
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
        ]
    } else {
        vec![
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    };
    for i in search.iter() {
        if let Some(u) = grid.iter().position(|sec| {
            sec.colour == Colour::White && sec.series.is_none() && sec.x == i.0 && sec.y == i.1
        }) {
            return Some(u);
        }
    }
    None
}
