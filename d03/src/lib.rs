pub fn solve(input: &str) -> aoc_common::AocResult {
    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let parser = Parser::new(input.chars());

    let mut s1 = 0;
    let mut s2 = 0;

    for (enabled, product) in parser {
        s1 += product;
        if enabled {
            s2 += product;
        }
    }

    Ok(format!("d03/01 = {}, d03/02 = {}", s1, s2))
}

struct Parser<T>
where
    T: Iterator<Item = char>,
{
    input: T,
    enabled: bool,
}

impl<T> Parser<T>
where
    T: Iterator<Item = char>,
{
    fn new(input: T) -> Self {
        Self {
            input,
            enabled: true,
        }
    }
}

impl<T> Iterator for Parser<T>
where
    T: Iterator<Item = char>,
{
    type Item = (bool, u64);

    fn next(&mut self) -> Option<Self::Item> {
        let mut prefix = None;

        loop {
            let token = consume_to_token(prefix, &mut self.input);
            match token {
                Token::Do => {
                    self.enabled = true;
                }

                Token::Dont => {
                    self.enabled = false;
                }

                Token::Mul => {
                    let (n, c) = parse_number(&mut self.input);
                    match (n, c) {
                        (None, Some(inner)) => {
                            prefix = Some(inner);
                            continue;
                        }

                        (Some(_), Some(inner)) => {
                            if inner != ',' {
                                prefix = Some(inner);
                                continue;
                            }
                        }

                        _ => return None,
                    }
                    let n1 = n.unwrap();

                    let (n, c) = parse_number(&mut self.input);
                    match (n, c) {
                        (None, Some(inner)) => {
                            prefix = Some(inner);
                            continue;
                        }

                        (Some(_), Some(inner)) => {
                            if inner != ')' {
                                prefix = Some(inner);
                                continue;
                            }
                        }

                        _ => return None,
                    }
                    let n2 = n.unwrap();

                    return Some((self.enabled, n1 * n2));
                }

                Token::Eof => {
                    return None;
                }
            }
        }
    }
}

fn parse_number<T>(chars: &mut T) -> (Option<u64>, Option<char>)
where
    T: Iterator<Item = char>,
{
    let mut result = 0;
    let mut read = false;

    for (i, c) in chars.enumerate() {
        if i >= 4 {
            return (None, Some(c));
        }

        read = true;
        let n = c.to_digit(10).map(|n| n as u64);

        match n {
            Some(n) => result = result * 10 + n,
            None => {
                if i == 0 {
                    return (None, Some(c));
                } else {
                    return (Some(result), Some(c));
                }
            }
        }
    }

    if !read {
        (None, None)
    } else {
        (Some(result), None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Mul,
    Do,
    Dont,
    Eof,
}

fn consume_to_token<T>(prefix: Option<char>, chars: &mut T) -> Token
where
    T: Iterator<Item = char>,
{
    let mut current = None;
    let mul_token = ['m', 'u', 'l', '('];
    let do_token = ['d', 'o', '(', ')'];
    let dont_token = ['d', 'o', 'n', '\'', 't', '(', ')'];
    let mut index = 0;

    let iter = match prefix {
        Some(c) => std::iter::repeat(c).take(1).chain(chars),
        None => std::iter::repeat(' ').take(0).chain(chars),
    };

    for c in iter {
        match current {
            None => {
                if c == mul_token[0] {
                    current = Some(Token::Mul);
                } else if c == do_token[0] {
                    current = Some(Token::Do);
                }
                // first chars of don't is handled by do
                index = 1;
            }

            Some(Token::Mul) => {
                if c == mul_token[index] {
                    index += 1;
                } else {
                    current = None;
                }

                if index == mul_token.len() {
                    return Token::Mul;
                }
            }

            Some(Token::Do) => {
                if c == do_token[index] {
                    index += 1;
                } else if c == dont_token[index] {
                    current = Some(Token::Dont);
                    index += 1;
                } else {
                    current = None;
                }

                if index == do_token.len() {
                    return Token::Do;
                }
            }

            Some(Token::Dont) => {
                if c == dont_token[index] {
                    index += 1;
                } else {
                    current = None;
                }

                if index == dont_token.len() {
                    return Token::Dont;
                }
            }

            _ => current = None,
        }
    }

    Token::Eof
}
