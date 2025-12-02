use tracing::{debug, info, trace};

fn main() {
    let input = include_str!("./input1.txt");
    process(input);
}

fn process(input: &str) -> u64 {
    let mut sum: u64 = 0;
    for range in input.split(',') {
        if range.is_empty() {
            continue;
        }
        let range_split: Vec<&str> = range.split('-').collect();
        let left_bound = range_split[0]
            .trim()
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("Left bound invalid: {}", range_split[0]));

        let right_bound = range_split[1]
            .trim()
            .parse::<u64>()
            .unwrap_or_else(|_| panic!("Right bound invalid: {}", range_split[1]));

        info!(left_bound, right_bound, "Checking range");
        for i in left_bound..=right_bound {
            if is_invalid(i) {
                sum = sum.checked_add(i).expect("Overflow");
            }
        }
    }
    println!("Sum is {sum}");
    sum
}

fn is_invalid(value: u64) -> bool {
    let value_str = value.to_string();
    if value_str.len() % 2 != 0 {
        // Short circuit odd length numbers as they can't have a pair of matching strings.
        return false;
    }

    debug!(value, "Checking number");
    let left = &value_str[0..value_str.len() / 2];
    let right = &value_str[(value_str.len() / 2)..value_str.len()];
    trace!(left, right, "Comparing halves");
    if left == right {
        info!(value, "Invalid number found");
        return true;
    }

    return false;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_1() {
        let _ = tracing_subscriber::fmt::try_init();
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(process(input), 1227775554);
    }
    //#[test]
    //fn test_is_invalid() {
    //    let _ = tracing_subscriber::fmt::try_init();
    //    assert!(is_invalid(11))
    //}
}
