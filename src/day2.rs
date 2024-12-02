fn is_safe(iterator: &mut dyn Iterator<Item = i32>) -> bool {
    let first_element = iterator.next().unwrap();
    let second_element = iterator.next().unwrap();
    if first_element == second_element || (first_element - second_element).abs() > 3 {
        return false;
    }
    let is_ascending = first_element < second_element;
    let mut current_element = second_element;
    while let Some(next_element) = iterator.next() {
        if next_element == current_element {
            return false;
        }

        if is_ascending {
            if next_element < current_element {
                return false;
            }
        } else {
            if next_element > current_element {
                return false;
            }
        }

        let difference = (next_element - current_element).abs();

        if difference > 3 {
            return false;
        }

        current_element = next_element;
    }

    true
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let mut readings = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let is_safe = is_safe(&mut readings);

        if is_safe {
            sum += 1;
        }
    }

    sum
}

fn is_safe_naive(iterator: &mut dyn Iterator<Item = i32>) -> bool {
    let collection = iterator.collect::<Vec<i32>>();
    if is_safe(&mut collection.iter().copied()) {
        return true;
    }

    for i in 0..collection.len() {
        let mut without_i = collection
            .iter()
            .copied()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, v)| v);
        if is_safe(&mut without_i) {
            return true;
        }
    }

    false
}

// This produces two false positives, but is only slightly faster
// than the naive solution, so it’s probably not worth the effort to fix
fn _is_safe2(iterator: &mut dyn Iterator<Item = i32>) -> bool {
    let mut ascension_violations = 0;
    let mut descension_violations = 0;
    let mut equal_element_violations = 0;
    let mut large_difference_violations = 0;

    let mut current_element = iterator.next().unwrap();

    while let Some(next_element) = iterator.next() {
        if next_element == current_element {
            equal_element_violations += 1;
            if equal_element_violations > 1 {
                return false;
            }
            continue;
        }

        let difference = (next_element - current_element).abs();
        if difference > 3 {
            large_difference_violations += 1;
            if large_difference_violations > 1 {
                return false;
            }
            continue;
        }

        if next_element < current_element {
            descension_violations += 1;
        }

        if next_element > current_element {
            ascension_violations += 1;
        }

        current_element = next_element;
    }

    if ascension_violations > 1 && descension_violations > 1 {
        return false;
    }

    println!(
        "-> {}, {}, {}, {}",
        ascension_violations,
        descension_violations,
        equal_element_violations,
        large_difference_violations
    );

    let min_violation = ascension_violations.min(descension_violations);

    min_violation + equal_element_violations + large_difference_violations <= 1
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let lines = input.lines();

    let mut sum = 0;

    for line in lines {
        let mut readings = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let are_readings_safe = is_safe_naive(&mut readings);

        if are_readings_safe {
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn sample1() {
        assert_eq!(part1("1 2 3 4 5 6\n1 2 3 4 5 6"), 2);
        assert_eq!(part1("6 5 4 3 2 1\n1 2 1 1 2 1"), 1);
    }

    #[test]
    fn sample2() {
        assert_eq!(part1("1 7 8 9 10\n10 9 8 2 1"), 0);
        assert_eq!(part1("0 1 21 22\n0 1 4 5 6 7"), 1);
    }

    #[test]
    fn sample3() {
        assert_eq!(part1("1 1 2 3 4\n10 10 9 8 7"), 0);
        assert_eq!(part1("1 2 2 3 4\n10 9 9 8 7"), 0);
        assert_eq!(part1("1 2 3 3 4\n10 9 8 8 7"), 0);
    }

    #[test]
    fn sample4() {
        assert_eq!(part2("1 2 3 4 5 6\n1 2 3 4 5 6"), 2);
        assert_eq!(part2("6 5 4 3 2 1\n6 5 4 3 2 1"), 2);
    }

    #[test]
    fn sample5() {
        assert_eq!(part2("1 7 8 9 10\n10 9 8 2 7"), 2);
        assert_eq!(part2("3 2 3 4 5\n9 10 9 8 7"), 2);
    }

    #[test]
    fn problematic_sample() {
        assert_eq!(part2("70 73 72 74 74"), 0);
    }
}
