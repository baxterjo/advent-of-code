use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("./input1.txt");
    process(input);
}

fn process(input: &str) -> u64 {
    let mut count = 0u64;
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();

    'lines: for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let split: Vec<u64> = line
            .split('-')
            .map(|item| item.parse::<u64>().expect("Bad input"))
            .collect();
        if split.len() == 2 {
            ranges.push(split[0]..=split[1]);
        } else {
            for range in ranges.iter() {
                if range.contains(&split[0]) {
                    count += 1;
                    continue 'lines;
                }
            }
        }
    }
    println!("Count is {count}");
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "3-5\n\
                            10-14\n\
                            16-20\n\
                            12-18\n\n\
                            1\n\
                            5\n\
                            8\n\
                            11\n\
                            17\n\
                            32";
        assert_eq!(process(input), 3);
    }
}
