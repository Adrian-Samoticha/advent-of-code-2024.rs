// #[aoc(day1, part1, Bytes)]
// pub fn part1_bytes(input: &[u8]) -> i32 {
//     input.iter().fold(0, |sum, c| match c {
//         b'(' => sum + 1,
//         b')' => sum - 1,
//         _ => unreachable!(),
//     })
// }

use std::collections::HashMap;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = input.lines();

    let mut first_vector = Vec::new();
    let mut second_vector = Vec::new();

    for line in lines {
        let mut numbers = line.split_whitespace();
        let first_number_as_text = numbers.next().unwrap();
        let second_number_as_text = numbers.next().unwrap();

        let first_number = first_number_as_text.parse::<i32>().unwrap();
        let second_number = second_number_as_text.parse::<i32>().unwrap();

        first_vector.push(first_number);
        second_vector.push(second_number);
    }

    first_vector.sort();
    second_vector.sort();

    let mut sum = 0;
    for i in 0..first_vector.len() {
        sum += (first_vector[i] - second_vector[i]).abs();
    }

    sum
}

fn count_in_vector(vector: &Vec<usize>, value: usize) -> usize {
    let mut sum = 0;
    for i in 0..vector.len() {
        if vector[i] == value {
            sum += 1;
        }
    }
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let lines = input.lines();

    let mut first_vector: Vec<usize> = Vec::new();
    let mut second_vector: Vec<usize> = Vec::new();

    for line in lines {
        let mut numbers = line.split_whitespace();
        let first_number_as_text = numbers.next().unwrap();
        let second_number_as_text = numbers.next().unwrap();

        let first_number = first_number_as_text.parse::<usize>().unwrap();
        let second_number = second_number_as_text.parse::<usize>().unwrap();

        first_vector.push(first_number);
        second_vector.push(second_number);
    }

    let mut number_to_count: HashMap<usize, usize> = HashMap::new();
    let mut sum: usize = 0;
    for i in 0..first_vector.len() {
        let value = first_vector[i];

        if !number_to_count.contains_key(&value) {
            let count = count_in_vector(&second_vector, value);
            number_to_count.insert(value, count);
        }

        let count = number_to_count.get(&value).unwrap();
        sum += value * count;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
    }

    // ((( and (()(()( both result in floor 3.
    #[test]
    fn sample2() {
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
    }

    // ))((((( also results in floor 3.
    #[test]
    fn sample3() {
        assert_eq!(part1("))((((("), 3);
    }

    // ()) and ))( both result in floor -1 (the first basement level).
    #[test]
    fn sample4() {
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
    }

    // ))) and )())()) both result in floor -3.
    #[test]
    fn sample5() {
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample6() {
        assert_eq!(part2(")"), 1);
    }

    // ()()) causes him to enter the basement at character position 5.
    #[test]
    fn sample7() {
        assert_eq!(part2("()())"), 5);
    }
}
