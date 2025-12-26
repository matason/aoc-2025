use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    let mut pallet = HashSet::new();

    let _ = input
        .lines()
        .enumerate()
        .map(|line| {
            let (y, row) = line;
            row.trim()
                .chars()
                .enumerate()
                .map(|char| {
                    let (x, roll) = char;
                    if roll == '@' {
                        pallet.insert((x as i32, y as i32));
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    pallet.iter().fold(0, |rolls, position| {
        if is_accessible(*position, &pallet) {
            rolls + 1
        } else {
            rolls
        }
    })
}

fn is_accessible(coordinates: (i32, i32), pallet: &HashSet<(i32, i32)>) -> bool {
    let (x, y) = coordinates;

    let neighbours = [
        (x, y + 1),
        (x, y - 1),
        (x - 1, y + 1),
        (x - 1, y),
        (x - 1, y - 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
    ];

    let rolls = neighbours.iter().fold(0, |rolls, neighbour| {
        if pallet.get(neighbour).is_some() {
            rolls + 1
        } else {
            rolls
        }
    });

    rolls < 4
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
