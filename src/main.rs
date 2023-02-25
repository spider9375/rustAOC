mod day_1;

use std::fs;

fn main() {
}

fn first() {
    if let Some(first_part) = day_1::first_part() {
        println!("first part - {}", first_part);
    }
    if let Some(second_part) = day_1::second_part() {
        println!("second part - {}", second_part);
    }

}
