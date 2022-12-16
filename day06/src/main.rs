use std::fs;

mod ds;

use ds::Datastream;

fn main() {
    let s = fs::read_to_string("data/input.txt").unwrap();
    println!(
        "The start of the first packet ends at the {}th character.",
        Datastream::new(&s).first_packet().unwrap()
    );
    println!(
        "The start of the first message ends at the {}th character.",
        Datastream::new(&s).first_message().unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(
            Datastream::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
                .first_packet()
                .unwrap(),
            7
        );
        assert_eq!(
            Datastream::new("bvwbjplbgvbhsrlpgdmjqwftvncz")
                .first_packet()
                .unwrap(),
            5
        );
        assert_eq!(
            Datastream::new("nppdvjthqldpwncqszvftbrmjlhg")
                .first_packet()
                .unwrap(),
            6
        );
        assert_eq!(
            Datastream::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
                .first_packet()
                .unwrap(),
            10
        );
        assert_eq!(
            Datastream::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
                .first_packet()
                .unwrap(),
            11
        );
    }

    #[test]
    fn part2() {
        assert_eq!(
            Datastream::new("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
                .first_message()
                .unwrap(),
            19
        );
        assert_eq!(
            Datastream::new("bvwbjplbgvbhsrlpgdmjqwftvncz")
                .first_message()
                .unwrap(),
            23
        );
        assert_eq!(
            Datastream::new("nppdvjthqldpwncqszvftbrmjlhg")
                .first_message()
                .unwrap(),
            23
        );
        assert_eq!(
            Datastream::new("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
                .first_message()
                .unwrap(),
            29
        );
        assert_eq!(
            Datastream::new("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
                .first_message()
                .unwrap(),
            26
        );
    }
}
