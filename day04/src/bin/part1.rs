use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    let grid_dimensions = (input.find("\n").unwrap(), input.matches("\n").count() + 1);
    let mut grid = HashMap::with_capacity(grid_dimensions.0.strict_mul(grid_dimensions.1));

    let _ = input
        .lines()
        .enumerate()
        .map(|y| {
            y.1.trim()
                .chars()
                .enumerate()
                .map(|x| {
                    grid.insert((x.0 as i32, y.0 as i32), x.1);
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    grid.iter().fold(0, |rolls, square| {
        if *square.1 == '@' && is_accessible(*square.0, &grid) {
            rolls + 1
        } else {
            rolls
        }
    })
}

fn is_accessible(square: (i32, i32), grid: &HashMap<(i32, i32), char>) -> bool {
    let squares = vec![
        (square.0, square.1 + 1),
        (square.0, square.1 - 1),
        (square.0 - 1, square.1 + 1),
        (square.0 - 1, square.1),
        (square.0 - 1, square.1 - 1),
        (square.0 + 1, square.1 + 1),
        (square.0 + 1, square.1),
        (square.0 + 1, square.1 - 1),
    ];

    let dots = squares.iter().fold(0, |dots, square| {
        if grid.get(square).is_none_or(|x| *x == '.') {
            dots + 1
        } else {
            dots
        }
    });

    dots > 4
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn if_this_works_its_going_to_be_brilliant() {
        assert_eq!(
            13,
            run("..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.")
        );
    }
}
