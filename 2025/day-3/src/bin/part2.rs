use tracing::{debug, info, instrument, trace};

fn main() {
    let input = include_str!("./input1.txt");
    process(input);
}

fn process(input: &str) -> u128 {
    let mut output: u128 = 0;
    for (idx, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let line_bytes = line.trim().as_bytes();
        let mut out = "".to_string();
        look_ahead_max(&mut out, line_bytes);

        info!(out, len = out.len(), "Got out");
        output = output
            .checked_add(out.parse::<u128>().expect("Line returned bad output."))
            .expect("Overflow!")
    }

    println!("Output is {output}");
    output
}

fn look_ahead_max(out: &mut String, line_bytes: &[u8]) {
    // If the out string is 12 digits long return.
    if out.len() == 12 {
        return;
    }
    debug!(
        out,
        line = String::from_utf8_lossy(line_bytes).to_string(),
        line_len = line_bytes.len(),
        "Checking remaining string"
    );
    let mut this_max = 0;
    // Find the next max digit in the remaining line.
    for i in 0..line_bytes.len() - (11 - out.len()) {
        let this_num = (line_bytes[i] as char).to_digit(10).expect("Bad digit") as u64;
        trace!(
            this_max,
            this_num,
            this_num_bytes = line_bytes[i],
            "Checking for new max"
        );
        this_max = this_max.max(this_num);
    }
    let idx = line_bytes
        .iter()
        .position(|&item| (item as char).to_digit(10).expect("Bad digit") as u64 == this_max)
        .expect("Couldn't find digit I just found?");
    // Update out
    *out = format!("{out}{this_max}");
    look_ahead_max(out, &line_bytes[idx + 1..line_bytes.len()]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_works() {
        let _ = tracing_subscriber::fmt::try_init();
        let input = "987654321111111\n\
                            811111111111119\n\
                            234234234234278\n\
                            818181911112111";
        assert_eq!(process(input), 3121910778619)
    }
    #[test]
    fn sub_sample_works() {
        let _ = tracing_subscriber::fmt::try_init();
        let line = "234234234234278";
        let line_bytes = line.trim().as_bytes();
        let mut out = "".to_string();
        look_ahead_max(&mut out, line_bytes);

        info!(out, len = out.len(), "Got out");
        assert_eq!(
            out.parse::<u128>().expect("Line returned bad output."),
            434234234278
        );
    }
}
