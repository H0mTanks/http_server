use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug, PartialEq)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

//? a=1&b=2&c&d=&e===&d=7&d=abc
//* a => 1 */
//* b => 2 */
//* c => "" */
//* d => "", 7, abc */
//* e => == */
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        let mut remaining = s;
        loop {
            let tup_pair_remaining = get_next_pair(remaining).unwrap_or_else(|| ("", ""));
            let pair = tup_pair_remaining.0;
            remaining = tup_pair_remaining.1;

            if (pair, remaining) == ("", "") {
                break;
            }

            let (key, val) = get_key_value_from_pair(pair).unwrap();

            data.entry(key)
                .and_modify(|e| match *e {
                    Value::Multiple(ref mut vals) => vals.push(val),
                    Value::Single(single_val) => *e = Value::Multiple(vec![single_val, val]),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}

fn get_key_value_from_pair(s: &str) -> Option<(&str, &str)> {
    if s.len() == 0 {
        return None;
    }

    for (i, c) in s.char_indices() {
        if c == '=' {
            return Some((&s[..i], &s[i + 1..]));
        }
    }

    Some((s, ""))
}

fn get_next_pair(s: &str) -> Option<(&str, &str)> {
    if s.len() == 0 {
        return None;
    }

    for (i, c) in s.char_indices() {
        if c == '&' {
            return Some((&s[..i], &s[i + 1..]));
        }
    }

    Some((s, ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_pair_test() {
        let test_str = "a=1&b=2&c&d=&e===&d=7&d=abc";

        let (mut word, mut remaining) = get_next_pair(test_str).unwrap();
        assert_eq!(word, "a=1");
        assert_eq!(remaining, "b=2&c&d=&e===&d=7&d=abc");

        (word, remaining) = get_next_pair(remaining).unwrap();
        assert_eq!(word, "b=2");
        assert_eq!(remaining, "c&d=&e===&d=7&d=abc");

        (word, remaining) = get_next_pair(remaining).unwrap();
        assert_eq!(word, "c");
        assert_eq!(remaining, "d=&e===&d=7&d=abc");

        (word, remaining) = get_next_pair(remaining).unwrap();
        assert_eq!(word, "d=");
        assert_eq!(remaining, "e===&d=7&d=abc");

        (word, remaining) = get_next_pair(remaining).unwrap();
        assert_eq!(word, "e===");
        assert_eq!(remaining, "d=7&d=abc");

        (word, remaining) = get_next_pair(remaining).unwrap();
        assert_eq!(word, "d=7");
        assert_eq!(remaining, "d=abc");

        (word, remaining) = get_next_pair(remaining).unwrap();
        assert_eq!(word, "d=abc");
        assert_eq!(remaining, "");

        assert_eq!(get_next_pair(remaining), None);
    }

    #[test]
    fn get_key_value_from_pair_test() {
        let test_str = vec!["a=1", "b=2", "c", "d=", "e===", "d=7", "d=abc"];

        let (mut key, mut val) = get_key_value_from_pair(test_str[0]).unwrap();
        assert_eq!(key, "a");
        assert_eq!(val, "1");

        (key, val) = get_key_value_from_pair(test_str[1]).unwrap();
        assert_eq!(key, "b");
        assert_eq!(val, "2");

        (key, val) = get_key_value_from_pair(test_str[2]).unwrap();
        assert_eq!(key, "c");
        assert_eq!(val, "");

        (key, val) = get_key_value_from_pair(test_str[3]).unwrap();
        assert_eq!(key, "d");
        assert_eq!(val, "");

        (key, val) = get_key_value_from_pair(test_str[4]).unwrap();
        assert_eq!(key, "e");
        assert_eq!(val, "==");

        (key, val) = get_key_value_from_pair(test_str[5]).unwrap();
        assert_eq!(key, "d");
        assert_eq!(val, "7");

        (key, val) = get_key_value_from_pair(test_str[6]).unwrap();
        assert_eq!(key, "d");
        assert_eq!(val, "abc");

        assert_eq!(get_next_pair(""), None);
    }

    #[test]
    fn get_qs() {
        use Value::*;
        let s = "a=1&b=2&c&d=&e===&d=7&d=abc";

        let qs = QueryString::from(s);

        assert_eq!(qs.data["a"], Single("1"));
        assert_eq!(qs.data["b"], Single("2"));
        assert_eq!(qs.data["c"], Single(""));
        assert_eq!(qs.data["d"], Multiple(vec!["", "7", "abc"]));
        assert_eq!(qs.data["e"], Single("=="));
    }
}
