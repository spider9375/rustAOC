use itertools::Itertools;
use std::cmp::Reverse;
use crate::days::helpers;

pub fn second_part(input: &str) -> Option<u32> {
    top_3_from_vec(&helpers::str_to_vec(&input))
}

pub fn first_part(input: &str) -> Option<u32> {
    max_from_vector(&helpers::str_to_vec(&input))
}

fn top_3_from_vec(vec: &Vec<u32>) -> Option<u32> {
    Some(vec
        .iter()
        .sorted_by_key(|x| Reverse(*x))
        .take(3)
        .sum())
}

fn max_from_vector(vec: &Vec<u32>) -> Option<u32> {
    vec.iter()
        .max()
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn test_part_one() {
        let test_input = String::from("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");
        let result = first_part(&test_input);
        assert_eq!(result, Some(24000));
    }

    #[test]
    fn test_part_two() {
        let test_input = String::from("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000");

        let result = second_part(&test_input);

        assert_eq!(result, Some(45000));
    }
}
