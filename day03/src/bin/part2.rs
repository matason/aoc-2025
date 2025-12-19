fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i64 {
    input.lines().fold(0, |acc, mut bank| {
        let mut batteries: Vec<char> = Vec::with_capacity(12);

        while batteries.len() < 12 {
            let mut max = bank.chars().max().unwrap();
            let mut position = bank.chars().position(|x| x == max).unwrap();
            let mut split = bank.split_at(position);

            while position + (12 - batteries.len()) > bank.len() {
                max = split.0.chars().max().unwrap();
                position = split.0.chars().position(|x| x == max).unwrap();
                split = bank.split_at(position);
            }

            batteries.push(max);
            bank = split.1.split_at(1).1;
        }

        acc + batteries.iter().collect::<String>().parse::<i64>().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_this_works_its_going_to_be_brilliant() {
        assert_eq!(
            3121910778619,
            run("987654321111111
811111111111119
234234234234278
818181911112111")
        );
    }
}
