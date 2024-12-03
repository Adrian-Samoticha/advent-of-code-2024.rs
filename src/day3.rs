enum State {
    Nothing,
    M,
    U,
    L,
    OpeningBracket,
    FirstNumber0,
    FirstNumber1,
    FirstNumber2,
    Comma,
    SecondNumber0,
    SecondNumber1,
    SecondNumber2,
    ClosingBracket,
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let mut state = State::Nothing;

    let mut num1 = 0;
    let mut num2 = 0;
    let mut sum = 0;

    for c in input.chars() {
        if c == 'm' {
            state = State::M;
            num1 = 0;
            num2 = 0;
            continue;
        }

        match state {
            State::M => {
                state = match c {
                    'u' => State::U,
                    _ => State::Nothing,
                }
            }
            State::U => {
                state = match c {
                    'l' => State::L,
                    _ => State::Nothing,
                }
            }
            State::L => {
                state = match c {
                    '(' => State::OpeningBracket,
                    _ => State::Nothing,
                }
            }
            State::OpeningBracket => {
                state = match c {
                    '0'..='9' => {
                        num1 = c.to_digit(10).unwrap() as i32;
                        State::FirstNumber0
                    }
                    ',' => State::Comma,
                    _ => State::Nothing,
                }
            }
            State::FirstNumber0 => {
                state = match c {
                    '0'..='9' => {
                        num1 = num1 * 10 + c.to_digit(10).unwrap() as i32;
                        State::FirstNumber1
                    }
                    ',' => State::Comma,
                    _ => State::Nothing,
                }
            }
            State::FirstNumber1 => {
                state = match c {
                    '0'..='9' => {
                        num1 = num1 * 10 + c.to_digit(10).unwrap() as i32;
                        State::FirstNumber2
                    }
                    ',' => State::Comma,
                    _ => State::Nothing,
                }
            }
            State::FirstNumber2 => {
                state = match c {
                    ',' => State::Comma,
                    _ => State::Nothing,
                }
            }
            State::Comma => {
                state = match c {
                    '0'..='9' => {
                        num2 = c.to_digit(10).unwrap() as i32;
                        State::SecondNumber0
                    }
                    _ => State::Nothing,
                }
            }
            State::SecondNumber0 => {
                state = match c {
                    '0'..='9' => {
                        num2 = num2 * 10 + c.to_digit(10).unwrap() as i32;
                        State::SecondNumber1
                    }
                    ')' => {
                        sum += num1 * num2;
                        State::ClosingBracket
                    }
                    _ => State::Nothing,
                }
            }
            State::SecondNumber1 => {
                state = match c {
                    '0'..='9' => {
                        num2 = num2 * 10 + c.to_digit(10).unwrap() as i32;
                        State::SecondNumber2
                    }
                    ')' => {
                        sum += num1 * num2;
                        State::ClosingBracket
                    }
                    _ => State::Nothing,
                }
            }
            State::SecondNumber2 => {
                state = match c {
                    ')' => {
                        sum += num1 * num2;
                        State::ClosingBracket
                    }
                    _ => State::Nothing,
                }
            }
            State::ClosingBracket => {
                state = State::Nothing;
            }
            State::Nothing => {}
        }
    }

    sum
}

enum State2 {
    Nothing,
    M,
    U,
    L,
    OpeningBracketForMul,
    FirstNumber0,
    FirstNumber1,
    FirstNumber2,
    Comma,
    SecondNumber0,
    SecondNumber1,
    SecondNumber2,
    ClosingBracketForMul,
    D,
    O,
    N,
    APOSTROPHE,
    T,
    OpeningBracketForDo,
    ClosingBracketForDo,
    OpeningBracketForDont,
    ClosingBracketForDont,
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let mut state = State2::Nothing;

    let mut is_enabled = true;
    let mut num1 = 0;
    let mut num2 = 0;
    let mut sum = 0;

    for c in input.chars() {
        if c == 'm' {
            state = State2::M;
            num1 = 0;
            num2 = 0;
            continue;
        }

        if c == 'd' {
            state = State2::D;
            continue;
        }

        match state {
            State2::M => {
                state = match c {
                    'u' => State2::U,
                    _ => State2::Nothing,
                }
            }
            State2::U => {
                state = match c {
                    'l' => State2::L,
                    _ => State2::Nothing,
                }
            }
            State2::L => {
                state = match c {
                    '(' => State2::OpeningBracketForMul,
                    _ => State2::Nothing,
                }
            }
            State2::OpeningBracketForMul => {
                state = match c {
                    '0'..='9' => {
                        if is_enabled {
                            num1 = c.to_digit(10).unwrap() as i32;
                        }
                        State2::FirstNumber0
                    }
                    ',' => State2::Comma,
                    _ => State2::Nothing,
                }
            }
            State2::FirstNumber0 => {
                state = match c {
                    '0'..='9' => {
                        if is_enabled {
                            num1 = num1 * 10 + c.to_digit(10).unwrap() as i32;
                        }
                        State2::FirstNumber1
                    }
                    ',' => State2::Comma,
                    _ => State2::Nothing,
                }
            }
            State2::FirstNumber1 => {
                state = match c {
                    '0'..='9' => {
                        if is_enabled {
                            num1 = num1 * 10 + c.to_digit(10).unwrap() as i32;
                        }
                        State2::FirstNumber2
                    }
                    ',' => State2::Comma,
                    _ => State2::Nothing,
                }
            }
            State2::FirstNumber2 => {
                state = match c {
                    ',' => State2::Comma,
                    _ => State2::Nothing,
                }
            }
            State2::Comma => {
                state = match c {
                    '0'..='9' => {
                        if is_enabled {
                            num2 = c.to_digit(10).unwrap() as i32;
                        }
                        State2::SecondNumber0
                    }
                    _ => State2::Nothing,
                }
            }
            State2::SecondNumber0 => {
                state = match c {
                    '0'..='9' => {
                        if is_enabled {
                            num2 = num2 * 10 + c.to_digit(10).unwrap() as i32;
                        }
                        State2::SecondNumber1
                    }
                    ')' => {
                        if is_enabled {
                            sum += num1 * num2;
                        }
                        State2::ClosingBracketForMul
                    }
                    _ => State2::Nothing,
                }
            }
            State2::SecondNumber1 => {
                state = match c {
                    '0'..='9' => {
                        if is_enabled {
                            num2 = num2 * 10 + c.to_digit(10).unwrap() as i32;
                        }
                        State2::SecondNumber2
                    }
                    ')' => {
                        if is_enabled {
                            sum += num1 * num2;
                        }
                        State2::ClosingBracketForMul
                    }
                    _ => State2::Nothing,
                }
            }
            State2::SecondNumber2 => {
                state = match c {
                    ')' => {
                        if is_enabled {
                            sum += num1 * num2;
                        }
                        State2::ClosingBracketForMul
                    }
                    _ => State2::Nothing,
                }
            }
            State2::ClosingBracketForMul => {
                state = State2::Nothing;
            }
            State2::Nothing => {}
            State2::D => {
                state = match c {
                    'o' => State2::O,
                    _ => State2::Nothing,
                };
            }
            State2::O => {
                state = match c {
                    'n' => State2::N,
                    '(' => State2::OpeningBracketForDo,
                    _ => State2::Nothing,
                };
            }
            State2::N => {
                state = match c {
                    '\'' => State2::APOSTROPHE,
                    _ => State2::Nothing,
                };
            }
            State2::APOSTROPHE => {
                state = match c {
                    't' => State2::T,
                    _ => State2::Nothing,
                };
            }
            State2::T => {
                state = match c {
                    '(' => State2::OpeningBracketForDont,
                    _ => State2::Nothing,
                };
            }
            State2::OpeningBracketForDont => {
                state = match c {
                    ')' => {
                        is_enabled = false;

                        State2::ClosingBracketForDont
                    }
                    _ => State2::Nothing,
                };
            }
            State2::ClosingBracketForDont => {
                state = State2::Nothing;
            }
            State2::OpeningBracketForDo => {
                state = match c {
                    ')' => {
                        is_enabled = true;
                        State2::ClosingBracketForDo
                    }
                    _ => State2::Nothing,
                };
            }
            State2::ClosingBracketForDo => {
                state = State2::Nothing;
            }
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
