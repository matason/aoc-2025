fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> usize {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let mut database: Vec<(i64, i64)> = Vec::new();

    for range in ranges.lines() {
        let (start, end) = range.split_once("-").unwrap();
        database.push((start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()));
    }

    ingredients
        .lines()
        .filter_map(|ingredient| ingredient.parse::<i64>().ok())
        .filter(|ingredient| {
            database
                .iter()
                .any(|(start, end)| ingredient >= start && ingredient <= end)
        })
        .count()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn if_this_works_its_going_to_be_brilliant() {
        assert_eq!(
            3,
            run("3-5
10-14
16-20
12-18

1
5
8
11
17
32")
        );
    }
}
