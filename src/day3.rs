use memchr::memmem;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let needle = "mul(";

    let mut it = memmem::find_iter(input.as_bytes(), needle);

    let comma_finder = memmem::Finder::new(",");
    let closing_bracket_finder = memmem::Finder::new(")");

    let mut sum = 0;
    while let Some(pos) = it.next() {
        let window_start_pos = pos + needle.len();
        let window_end_pos = window_start_pos + 8;
        let window = &input[window_start_pos..window_end_pos];

        let comma_pos = comma_finder.find(window.as_bytes());
        let closing_bracket_pos = closing_bracket_finder.find(window.as_bytes());

        if let (Some(comma_pos), Some(closing_bracket_pos)) = (comma_pos, closing_bracket_pos) {
            let number1 = window[0..comma_pos].parse::<i32>();
            let number2 = window[(comma_pos + 1)..closing_bracket_pos].parse::<i32>();

            if number1.is_err() || number2.is_err() {
                continue;
            }
            sum += number1.unwrap() * number2.unwrap();
        }
    }

    sum
}

enum PotentiallyConsumedPosition {
    NonConsumed(Option<usize>),
    Consumed,
}

impl PartialEq for PotentiallyConsumedPosition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                PotentiallyConsumedPosition::NonConsumed(a),
                PotentiallyConsumedPosition::NonConsumed(b),
            ) => a == b,
            (PotentiallyConsumedPosition::Consumed, PotentiallyConsumedPosition::Consumed) => true,
            _ => false,
        }
    }
}

fn is_optional_pos_smaller(a: Option<usize>, b: Option<usize>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a < b,
        (Some(_), None) => true,
        _ => false,
    }
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut mul_it = memmem::find_iter(input.as_bytes(), "mul(");
    let mut do_it = memmem::find_iter(input.as_bytes(), "do()");
    let mut dont_it = memmem::find_iter(input.as_bytes(), "don't()");

    let comma_finder = memmem::Finder::new(",");
    let closing_bracket_finder = memmem::Finder::new(")");

    let mut is_mul_enabled = true;

    let mut sum = 0;

    let mut mul_pos = PotentiallyConsumedPosition::NonConsumed(mul_it.next());
    let mut do_pos = PotentiallyConsumedPosition::NonConsumed(do_it.next());
    let mut dont_pos = PotentiallyConsumedPosition::NonConsumed(dont_it.next());

    let mut find_next_pos = || -> Option<usize> {
        if mul_pos == PotentiallyConsumedPosition::Consumed {
            mul_pos = PotentiallyConsumedPosition::NonConsumed(mul_it.next());
        }
        if do_pos == PotentiallyConsumedPosition::Consumed {
            do_pos = PotentiallyConsumedPosition::NonConsumed(do_it.next());
        }
        if dont_pos == PotentiallyConsumedPosition::Consumed {
            dont_pos = PotentiallyConsumedPosition::NonConsumed(dont_it.next());
        }

        let mut result = None;

        if let PotentiallyConsumedPosition::NonConsumed(mul_pos_value) = mul_pos {
            if let PotentiallyConsumedPosition::NonConsumed(do_pos_value) = do_pos {
                if let PotentiallyConsumedPosition::NonConsumed(dont_pos_value) = dont_pos {
                    if is_optional_pos_smaller(mul_pos_value, do_pos_value)
                        && is_optional_pos_smaller(mul_pos_value, dont_pos_value)
                    {
                        result = mul_pos_value;
                        mul_pos = PotentiallyConsumedPosition::Consumed;
                    } else if is_optional_pos_smaller(do_pos_value, mul_pos_value)
                        && is_optional_pos_smaller(do_pos_value, dont_pos_value)
                    {
                        result = do_pos_value;
                        do_pos = PotentiallyConsumedPosition::Consumed;
                    } else if is_optional_pos_smaller(dont_pos_value, mul_pos_value)
                        && is_optional_pos_smaller(dont_pos_value, do_pos_value)
                    {
                        result = dont_pos_value;
                        dont_pos = PotentiallyConsumedPosition::Consumed;
                    }
                }
            }
        }

        return result;
    };

    while let Some(pos) = find_next_pos() {
        if input.as_bytes()[pos] == b'm' {
            if is_mul_enabled {
                let window_start_pos = pos + "mul(".len();
                let window_end_pos = window_start_pos + 8;
                let window = &input[window_start_pos..window_end_pos];

                let comma_pos = comma_finder.find(window.as_bytes());
                let closing_bracket_pos = closing_bracket_finder.find(window.as_bytes());

                if let (Some(comma_pos), Some(closing_bracket_pos)) =
                    (comma_pos, closing_bracket_pos)
                {
                    let number1 = window[0..comma_pos].parse::<i32>();
                    let number2 = window[(comma_pos + 1)..closing_bracket_pos].parse::<i32>();

                    if number1.is_err() || number2.is_err() {
                        continue;
                    }
                    sum += number1.unwrap() * number2.unwrap();
                }
            }
        } else if input.as_bytes()[pos + 2] == b'(' {
            is_mul_enabled = true;
        } else if input.as_bytes()[pos + 2] == b'n' {
            is_mul_enabled = false;
        } else {
            continue;
        }
    }

    sum
}

#[cfg(test)]
mod tests {}
