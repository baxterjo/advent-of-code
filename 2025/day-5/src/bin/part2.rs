use std::{ops::RangeInclusive, u64};

use tracing::{debug, info, info_span, trace, warn};

fn main() {
    let _ = tracing_subscriber::fmt::try_init();
    let input = include_str!("./input1.txt");
    process(input);
}

fn process(input: &str) -> u64 {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();

    'lines: for (line_no, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let span = info_span!("line", line_no);
        let _enter = span.enter();

        let mut split: Vec<u64> = line
            .split('-')
            .map(|item| item.parse::<u64>().expect("Bad input"))
            .collect();
        // Sort the two bounds in case of funny business.
        split.sort();
        if split.len() == 2 {
            info!(start = split[0], end = split[1], "Range found");
            // Look through existing ranges to see if they already cover numbers in new range.
            for range in ranges.iter_mut() {
                if range.contains(&split[0]) && range.contains(&split[1]) {
                    // If any range contains both ends of the new range, those numbers are already
                    // covered. Skip.
                    trace!(conflict_range=?range,"Range skipped");
                    continue 'lines;
                } else if range.contains(&split[0]) && !range.contains(&split[1]) {
                    // If range contains left bound but not right bound, move left bound to end of
                    // range + 1.
                    let new = range.end() + 1;
                    debug!(trimmed = new - split[0], conflict_range=?range, "Trimmed from start");
                    split[0] = new;
                } else if !range.contains(&split[0]) && range.contains(&split[1]) {
                    // If range contains right bound but not left bound, move right bound to
                    // beginning of range - 1.
                    let new = range.start() - 1;
                    debug!(trimmed = split[1] - new, conflict_range=?range, "Trimmed from end");
                    split[1] = new;
                } else if split[0] < *range.start() && *range.end() < split[1] {
                    // If the incoming range completely eclipses the old range. Replace it in
                    // place.

                    warn!("Modifying in place");
                    *range = split[0]..=split[1];
                    continue 'lines;
                }
                // If range does not contain either bounds, it is a fresh set of numbers.
            }
            warn!(start = split[0], end = split[1], "Pushing range");
            ranges.push(split[0]..=split[1]);
            ranges.sort_by_key(|a| *a.start());
        }
    }
    info!(range_count = ranges.len());
    // Ideally the ranges should be deconflicted by this point, so a sort should have no overlap.
    info!("{:#?}", ranges);
    let mut count = 0u64;
    for i in 0..ranges.len() {
        if i != 0 {
            let last_max = ranges
                .get(i.saturating_sub(1))
                .map(|r| *r.end())
                .unwrap_or(0);
            assert!(
                last_max < *ranges[i].start(),
                "Range {} start collides with {} end",
                i,
                i - 1
            );
        }
        let next_min = ranges.get(i + 1).map(|r| *r.end()).unwrap_or(u64::MAX);

        assert!(
            next_min > *ranges[i].end(),
            "Range {} end collides with {} start",
            i,
            i - 1
        );
        count = count
            .checked_add(ranges[i].end() - ranges[i].start() + 1)
            .expect("Overflow!");
    }

    println!("Count is {count}");
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let _ = tracing_subscriber::fmt::try_init();
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
        assert_eq!(process(input), 14);
    }
}
