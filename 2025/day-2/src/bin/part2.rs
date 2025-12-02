use tracing::{debug, info, trace};

fn main() {
    let input = include_str!("./input2.txt");
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

    debug!(value, "Checking number");
    let half_length = value_str.len() / 2;
    for i in 0..half_length {
        // Two windows growing in size to eventually cover the entire value.
        let left = &value_str[0..=i];
        let right = &value_str[i + 1..=i + left.len()];
        trace!(left, right, "Comparing halves");
        if left == right {
            // At this point there is a potential to match. So now we need to make sure the
            // remainder of the value only contains the potential matches.
            if value_str.replace(left, "").is_empty() {
                info!(value, "Invalid number found");

                return true;
            }
        }
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
        assert_eq!(process(input), 4174379265);
    }
    //#[test]
    //fn test_is_invalid() {
    //    let _ = tracing_subscriber::fmt::try_init();
    //    assert!(is_invalid(11))
    //}
}
