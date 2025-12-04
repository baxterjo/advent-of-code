use tracing::{debug, info, trace};

fn main() {
    let input = include_str!("./input1.txt");
    process(input);
}

fn process(input: &str) -> u64 {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.trim().chars().collect())
            }
        })
        .collect();

    info!(rows = grid.len(), columns = grid[0].len(), "Grid shape");

    let mut rolls = 0;
    loop {
        let this_pass = sweep_rolls(&mut grid);
        if this_pass == 0 {
            break;
        }
        rolls += this_pass;
    }

    println!("Count is {rolls}");
    rolls
}

fn sweep_rolls(grid: &mut Vec<Vec<char>>) -> u64 {
    let mut rolls = 0u64;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            // Only evaluate rolls
            if grid[y][x] != '@' {
                continue;
            }
            let count = count_surrounding_rolls(&grid, x, y);
            if count < 4 {
                grid[y][x] = 'x';
                rolls += 1;
            }
        }
    }
    rolls
}

fn count_surrounding_rolls(grid: &Vec<Vec<char>>, x: usize, y: usize) -> u64 {
    debug!(y, x, "Evaluating roll");
    let mut count = 0u64;

    for i in y.saturating_sub(1)..=y + 1 {
        for j in x.saturating_sub(1)..=x + 1 {
            if i == y && x == j {
                // This is the roll in question, skip
                continue;
            }

            let add = grid
                .get(i)
                .map(|line| line.get(j))
                .flatten()
                .map(|character| if *character == '@' { 1 } else { 0 });

            trace!(val=?add, y = i, x = j, "Checking neighbor");
            count += add.unwrap_or(0);
        }
    }
    debug!(count, "Surrounding rolls");
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let _ = tracing_subscriber::fmt::try_init();
        let input = " ..@@.@@@@.\n\
                            @@@.@.@.@@\n\
                            @@@@@.@.@@\n\
                            @.@@@@..@.\n\
                            @@.@@@@.@@\n\
                            .@@@@@@@.@\n\
                            .@.@.@.@@@\n\
                            @.@@@.@@@@\n\
                            .@@@@@@@@.\n\
                            @.@.@@@.@.";

        assert_eq!(process(input), 43);
    }
}
