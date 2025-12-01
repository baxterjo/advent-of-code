use tracing::{debug, info};
fn main() {
    let _ = tracing_subscriber::fmt::try_init();
    let input = include_str!("./input1.txt");
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
        dial = turn_dial(dial, line);
        if dial == 0 {
            count += 1;
        }
    }

    info!("Count is {count}");
    count
}

fn turn_dial(dial: i64, input: &str) -> i64 {
    let (direction, operand) = input.split_at(1);
    let operand: i64 = operand.parse::<i64>().expect("Bad parse");

    let operand = operand % 100;

    // Mod 100 here strips the unecessary full rotations from the input number.
    let pre_result = match direction {
        "L" => dial - operand,
        "R" => dial + operand,
        other => panic!("Unexpected input direction: {other}"),
    };

    let result = if pre_result < 0 {
        pre_result + 100
    } else {
        pre_result % 100
    };

    debug!("Started at {dial}, turned {input}, pre_result: {pre_result}, result: {result}");

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        let test_input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(process(test_input), 3);
    }
    #[test]
    fn test_case_1() {
        let _ = tracing_subscriber::fmt::try_init();
        assert_eq!(turn_dial(0, "L99"), 1);
    }
}
