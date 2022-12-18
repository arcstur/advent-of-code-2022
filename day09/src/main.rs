use std::fs;
use std::io::Error;

mod rope;

use rope::RopeHistory;

fn main() -> Result<(), Error> {
    let string = fs::read_to_string("data/input.txt")?;
    let history = RopeHistory::from_input(&string);
    println!(
        "The tail visited, at least once, {} positions",
        history.tail_unique_positions()
    );

    assert_eq!(history.tail_unique_positions(), 6023);

    let knot_count = 10;
    let history = RopeHistory::from_input_with(&string, knot_count).unwrap();
    println!(
        "With {} knots, however, the tail visited, at least once, {} positions",
        knot_count,
        history.tail_unique_positions()
    );

    assert_eq!(history.tail_unique_positions(), 2533);

    Ok(())
}
