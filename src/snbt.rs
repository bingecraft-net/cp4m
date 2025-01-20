use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(i64),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEndOfInput,
    UnmatchedLeftBrace,
    UnmatchedRightBrace,
}

pub fn parse_snbt(input: String) -> Result<HashMap<String, Value>, Error> {
    let mut result = HashMap::new();
    let mut state = State::Start;
    let mut index = 0;
    let chars: Vec<char> = input.chars().collect();
    while index < chars.len() {
        if chars[index].is_whitespace() {
            index += 1;
        } else {
            match (&state, chars[index]) {
                (State::Start, '{') => state = State::Object,
                (State::Object, '}') => state = State::End,
                (_, '}') => return Err(Error::UnmatchedRightBrace),
                (State::Object, _) => {
                    let key = read_until(&chars, &mut index, ':');
                    index += 2;
                    let mut value = read_until(&chars, &mut index, '\n');
                    if value[value.len() - 1] == 'L' {
                        value.pop();
                    }
                    let number = value.iter().collect::<String>().parse().unwrap();
                    result.insert(key.iter().collect(), Value::Number(number));
                }
                _ => todo!("got {:?}", chars[index]),
            }
            index += 1;
        }
    }
    match state {
        State::Start => Err(Error::UnexpectedEndOfInput),
        State::Object => Err(Error::UnmatchedLeftBrace),
        State::End => Ok(result),
    }
}

#[derive(PartialEq)]
enum State {
    Start,
    Object,
    End,
}

fn read_until(chars: &Vec<char>, index: &mut usize, until: char) -> Vec<char> {
    let k = *index;
    while *index < chars.len() && chars[*index] != until {
        *index += 1;
    }
    chars[k..*index].to_vec()
}

#[cfg(test)]
mod tests {
    use super::{parse_snbt, Error, Value};
    use std::collections::HashMap;

    #[test]
    fn blank() {
        let input = String::from("");
        let actual = parse_snbt(input).unwrap_err();
        let expected = Error::UnexpectedEndOfInput;
        assert_eq!(actual, expected);
    }

    #[test]
    fn unmatched_left_brace() {
        let input = String::from("{");
        let actual = parse_snbt(input).unwrap_err();
        let expected = Error::UnmatchedLeftBrace;
        assert_eq!(actual, expected);
    }

    #[test]
    fn unmatched_right_brace() {
        let input = String::from("}");
        let actual = parse_snbt(input).unwrap_err();
        let expected = Error::UnmatchedRightBrace;
        assert_eq!(actual, expected);
    }

    #[test]
    fn empty_object() {
        let input = String::from("{}");
        let actual = parse_snbt(input).unwrap();
        let expected = HashMap::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn key_value_number() {
        let input = String::from("{\n\t\tmax_claim_chunks: 500\n\t\tmax_force_load_chunks: 25\n}");
        let actual = parse_snbt(input).unwrap();
        let expected = HashMap::from([
            (String::from("max_claim_chunks"), Value::Number(500)),
            (String::from("max_force_load_chunks"), Value::Number(25)),
        ]);
        assert_eq!(actual, expected);
    }

    #[test]
    fn key_value_long() {
        let input = String::from("{\n\t\tlast_login_time: 12345L\n}");
        let actual = parse_snbt(input).unwrap();
        let expected = HashMap::from([(String::from("last_login_time"), Value::Number(12345))]);
        assert_eq!(actual, expected);
    }
}
