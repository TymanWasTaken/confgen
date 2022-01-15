const TRUE_INPUTS: [&str; 9] = [
    "yes",
    "true",
    "y",
    "yea",
    "yeah",
    "yep",
    "yup",
    "1",
    "on"
];
const FALSE_INPUTS: [&str; 6] = [
    "no",
    "false",
    "n",
    "nope",
    "0",
    "off"
];

fn includes(value: &str, list: &[&str]) -> bool {
    list.iter().any(|&x| value.to_lowercase().contains(x))
}

pub mod parsing {
    use crate::parsing_utils::*;

    pub(crate) fn number(str: &str) -> Option<i32> {
        return str.parse::<i32>().ok();
    }

    pub(crate) fn boolean(str: &str) -> Option<bool> {
        return match str {
            _ if includes(str, &TRUE_INPUTS) => Some(true),
            _ if includes(str, &FALSE_INPUTS) => Some(false),
            _ => None,
        };
    }
}