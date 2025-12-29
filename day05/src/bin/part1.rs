fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let mut database: Vec<(i64, i64)> = Vec::new();

    let _ = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once("-").unwrap();
            database.push((start.parse::<i64>().unwrap(), end.parse::<i64>().unwrap()));
        })
        .collect::<Vec<_>>();

    ingredients.lines().fold(0, |fresh, ingredient| {
        let found = database.iter().any(|(start, end)| {
            ingredient.parse::<i64>().unwrap() >= *start
                && ingredient.parse::<i64>().unwrap() <= *end
        });
        if found { fresh + 1 } else { fresh }
    })
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
