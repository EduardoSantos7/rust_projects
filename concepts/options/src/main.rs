use std::num::ParseIntError;

#[derive(Debug)]
struct SummationError;

fn sum_str_vec(strings: Vec<String>) -> Result<String, SummationError> {
    let mut accumulator: i32 = 0;

    for s in strings {
        // accumulator += match to_int(&s) {
        //     Some(val) => val,
        //     None => 0
        // }
        // if let Some(val) = to_int(&s) {
        //     accumulator += val;
        // }
        //accumulator += to_int(&s).ok_or(SummationError)?;
        // accumulator += to_int(&s)?; 
        accumulator += to_int(&s).map_err(|_| SummationError)?;
    }

    Ok(accumulator.to_string())
}

fn to_int(s: &str) -> Result<i32, ParseIntError> {
    // s.parse().unwrap() // panic if there is an issue
    // s.parse().expect("issue converting string to int") // similar to unwrap but let you add a message
    // s.parse().unwrap_or(0) // return 0 as default if anything fails
    // s.parse().ok() // return an option that can be None or Some, and need to be handled by caller
    s.parse()
}

fn main() {
    let strings = vec!["1".to_string(), "abc".to_string(), "3".to_string()];
    let sum = sum_str_vec(strings);
    println!("Sum: {:?}", sum);
}