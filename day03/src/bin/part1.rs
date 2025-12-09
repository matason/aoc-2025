fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    input.lines().fold(0, |acc, bank| {
        let mut number = vec![];
        number.push(bank.chars().max().unwrap());
        let split = bank.split_once(number[0]).unwrap();
        if split.1.is_empty() {
            number.insert(0, split.0.chars().max().unwrap());
        } else {
            number.push(split.1.chars().max().unwrap());
        }

        let mut joltage = String::from(number[0]);
        joltage.push(number[1]);
        acc + joltage.parse::<i32>().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_this_works_its_going_to_be_brilliant() {
        assert_eq!(
            357,
            run("987654321111111
811111111111119
234234234234278
818181911112111")
        );
    }
}
