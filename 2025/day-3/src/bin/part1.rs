fn main() {
    let input = include_str!("./input1.txt");
    process(input);
}

fn process(input: &str) -> u64 {
    let mut output = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        output += find_max(line);
    }

    println!("Output is {output}");
    output
}

fn find_max(line: &str) -> u64 {
    let mut out = 0;

    let line_bytes = line.trim().as_bytes();

    for i in 0..line_bytes.len() - 1 {
        for j in i + 1..line_bytes.len() {
            out = out.max(
                format!("{}{}", line_bytes[i] as char, line_bytes[j] as char)
                    .parse::<u64>()
                    .expect("Invalid character."),
            );
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample_works() {
        let input = "987654321111111\n\
                            811111111111119\n\
                            234234234234278\n\
                            818181911112111";
        assert_eq!(process(input), 357)
    }
}
