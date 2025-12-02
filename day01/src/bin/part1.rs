fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    let mut position = 50;
    input
        .lines()
        .filter(|turn| {
            let instruction = turn.split_at(1);
            let mut clicks = instruction.1.parse::<i32>().unwrap();
            if clicks > 100 {
                clicks = clicks % 100;
            }
            match instruction.0 {
                "L" => {
                    position -= clicks;
                    if position < 0 {
                        position += 100;
                    }
                }
                "R" => {
                    position += clicks;
                    if position > 99 {
                        position -= 100;
                    }
                }
                _ => panic!(),
            }
            position == 0
        })
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_this_works_its_going_to_be_brilliant() {
        assert_eq!(
            3,
            run("L68
L30
R48
L5
R60
L55
L1
L99
R14
L82")
        );
    }
}
