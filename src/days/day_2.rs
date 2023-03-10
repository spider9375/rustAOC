use std::fmt::Formatter;

pub fn first_part(input: &str) -> Option<u32> {
   calculate_points_first_part(input)
}

pub fn second_part(input: &str) -> Option<u32> {
    calculate_points_second_part(input)
}
#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
enum Res {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

#[derive(PartialEq, Copy, Clone)]
#[repr(u32)]
enum Type {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn match_input(input: char) -> Result<Type, ()> {
    match input {
        'X' => Ok(Type::Rock),
        'A' => Ok(Type::Rock),
        'B' => Ok(Type::Paper),
        'Y' => Ok(Type::Paper),
        'C' => Ok(Type::Scissors),
        'Z' => Ok(Type::Scissors),
        _ => Err(())
    }
}

fn calc_res_part_1(enemy: Type, player: Type) -> Res {
    match player {
        Type::Rock => if enemy == Type::Scissors { Res::Win } else if enemy == Type::Paper {Res::Lose} else { Res::Draw},
        Type::Paper => if enemy == Type::Rock { Res::Win } else if enemy == Type::Scissors {Res::Lose} else { Res::Draw},
        Type::Scissors => if enemy == Type::Paper { Res::Win } else if enemy == Type::Rock {Res::Lose} else { Res::Draw},
    }
}

fn calculate_points_first_part(input: &str) -> Option<u32> {
    let vec_of_lines = input.split('\n');
    let result = vec_of_lines
        .map(|line| line
            .split(' ')
            .filter_map(|c| match_input(c.parse::<char>().ok().unwrap()).ok())
            .collect::<Vec<Type>>())
        .map(|vec| build_tuple_from_vec(vec))
        .map(|(a,b)|  calc_res_part_1(a,b) as u32 + b as u32)
        .sum();

    return Some(result);
}


fn calculate_points_second_part(input: &str) -> Option<u32> {
    let vec_of_lines = input.split('\n');
    let result = vec_of_lines
        .map(|line| line
            .split(' ')
            .filter_map(|c| c.parse::<char>().ok())
            .collect::<Vec<char>>())
        .map(|vec| {
            let (enemy, result) = match (vec.first(), vec.last()) {
                (Some(&enemy), Some(&result)) => (enemy, result),
                _ => panic!("Invalid")
            };

            let (enemy, result) = match (match_input(enemy).ok(), match_result(result).ok()) {
                (Some(enemy), Some(result)) => (enemy, result),
                _ => panic!("Invalid")
            };

            return (enemy, result);
        })
        .map(|(a, result)| figure_shape(a, result) as u32 + result as u32)
        .sum();

    return Some(result);
}

fn build_tuple_from_vec(vec: Vec<Type>) -> (Type, Type){
    let (first, last) = match (vec.first(), vec.last()) {
        (Some(&first), Some(&last)) => (first, last),
        _ => panic!("Vec must have first and last")
    };

    return (first, last);
}

fn match_result(input: char) -> Result<Res, ()> {
    match input {
        'X' => Ok(Res::Lose),
        'Y' => Ok(Res::Draw),
        'Z' => Ok(Res::Win),
        _ => Err(())
    }
}

fn figure_shape(enemy: Type, result: Res) -> Type {
    match enemy {
        Type::Rock => {if result == Res::Draw { Type::Rock } else if result == Res::Win { Type::Paper } else { Type::Scissors }}
        Type::Paper => {if result == Res::Draw { Type::Paper } else if result == Res::Win { Type::Scissors} else { Type::Rock}}
        Type::Scissors => {if result == Res::Draw { Type::Scissors } else if result == Res::Win { Type::Rock } else { Type::Paper}}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = String::from("A Y
B X
C Z");
        let result = first_part(&input);

        assert_eq!(result, Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = String::from("A Y
B X
C Z");
        let result = second_part(&input);

        assert_eq!(result, Some(12));
    }
}
