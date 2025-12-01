use tracing::{debug, info};
fn main() {
    let _ = tracing_subscriber::fmt::try_init();
    let input = include_str!("./input2.txt");
    process(input);
}

// Dial starts at 50 every time.
const START: i64 = 50;
pub fn process(input: &str) -> i64 {
    let mut dial = START;
    let mut count = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        dial = turn_dial(&mut count, dial, line);
        if dial == 0 {
            count += 1;
        }
    }

    info!("Count is {count}");
    count
}

fn turn_dial(count: &mut i64, dial: i64, input: &str) -> i64 {
    let (direction, operand) = input.split_at(1);
    let operand: i64 = operand.parse::<i64>().expect("Bad parse");
    let starting_count = *count;

    // Account for full rotations.
    let full_rotations = operand / 100;
    *count += full_rotations;

    let operand = operand % 100;

    // Mod 100 here strips the unecessary full rotations from the input number.
    let pre_result = match direction {
        "L" => dial - operand,
        "R" => dial + operand,
        other => panic!("Unexpected input direction: {other}"),
    };

    let mut zero_cross = false;
    let result = if pre_result < 0 {
        // If less than 0 then it crossed the 0 boundary
        if dial != 0 {
            *count += 1;
            zero_cross = true;
        }
        pre_result + 100
    } else {
        // If greater than 100 then it crossed the 0 b
        if dial != 0 && pre_result > 100 {
            zero_cross = true;
            *count += 1;
        }
        pre_result % 100
    };

    debug!(
        started = dial,
        starting_count, input, zero_cross, pre_result, result, count, "Turned dial"
    );

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        let _ = tracing_subscriber::fmt::try_init();
        let test_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(process(test_input), 6);
    }
    #[test]
    fn test_case_1() {
        let _ = tracing_subscriber::fmt::try_init();
        assert_eq!(turn_dial(&mut 0, 0, "L99"), 1);
    }
}
