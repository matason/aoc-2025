fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i64 {
    input.split(",").fold(0, |mut acc, range| {
        let endpoints = range.split_once("-");
        let blah = endpoints.unwrap();
        let start = blah.0.parse::<i64>().unwrap();
        let end = blah.1.parse::<i64>().unwrap();

        for id in start..=end {
            let number = id.to_string();
            if number.len() % 2 == 0 {
                let half = number.split_at(number.len() / 2);
                if half.0 == half.1 {
                    acc += id;
                }
            }
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_this_works_its_going_to_be_brilliant() {
        assert_eq!(
            1_227_775_554,
            run(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
            )
        );
    }
}
