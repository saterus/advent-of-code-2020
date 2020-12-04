use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

fn main() {
    println!("Hello from day-01!");

    let file_contents = load_file("assets/day-01-a.input").expect("Could not read puzzle file!");

    let mut list = parse_list(&file_contents).expect("Failed to load the list!");
    list.sort();

    match find_2020(&list) {
        Some((a, b, c)) => println!(
            "Found the answer: {a} + {b} + {c} = 2020! {a} * {b} * {c} = {product}",
            a = a,
            b = b,
            c = c,
            product = a * b * c
        ),
        None => println!("No answer found. :("),
    }
}

fn load_file<P: AsRef<Path>>(path: P) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn parse_list(lines: &str) -> Result<Vec<i32>, std::io::Error> {
    let numbers = lines
        .split("\n")
        .map(|s| i32::from_str(s).unwrap_or(0))
        .filter(|n| *n > 0 && *n < 2020)
        .collect::<Vec<i32>>();
    Ok(numbers)
}

fn find_2020(list: &[i32]) -> Option<(i32, i32, i32)> {
    'outer: for a in list.iter() {
        'middle: for b in list.iter() {
            if a + b > 2020 {
                continue 'outer;
            }

            'inner: for c in list.iter() {
                match (a + b + c).cmp(&2020) {
                    Ordering::Greater => continue 'middle,
                    Ordering::Equal => return Some((*a, *b, *c)),
                    Ordering::Less => continue 'inner,
                }
            }
        }
    }

    None
}
