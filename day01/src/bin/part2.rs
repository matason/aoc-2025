struct Dial {
    position: i32,
    hits: i32,
}

impl Dial {
    fn turn(self: &mut Dial, direction: Direction, clicks: i32) -> i32 {
        match direction {
            Direction::L => {
                for _ in 0..clicks {
                    self.position -= 1;
                    if self.position < 0 {
                        self.position += 100;
                    }
                    if self.position == 0 {
                        self.hits += 1;
                    }
                }
            }
            Direction::R => {
                for _ in 0..clicks {
                    self.position += 1;
                    if self.position > 99 {
                        self.position -= 100;
                    }
                    if self.position == 0 {
                        self.hits += 1;
                    }
                }
            }
        }
        self.hits
    }
}

enum Direction {
    L,
    R,
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", run(input.trim()))
}

fn run(input: &str) -> i32 {
    let mut dial = Dial {
        position: 50,
        hits: 0,
    };

    let mut hits = input
        .lines()
        .map(|turn| {
            let instruction = turn.split_at(1);
            let clicks = instruction.1.parse::<i32>().unwrap();

            let direction = match instruction.0 {
                "L" => Direction::L,
                "R" => Direction::R,
                _ => panic!(),
            };

            dial.turn(direction, clicks)
        })
        .collect::<Vec<_>>();

    hits.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_this_works_its_going_to_be_brilliant() {
        assert_eq!(
            6,
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
