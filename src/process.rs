use regex::Regex;

pub fn extract<'a>(data: &'a Vec<&str>, pattern: &'a Regex) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for value in data {
        let capture = pattern.captures(value);
        if let Some(value) = capture {
            let value = value.get(0).map_or("", |m| m.as_str());
            result.push(value);
        }
    }
    result
}

pub fn diff<'a>(a: &'a Vec<&str>, b: &'a Vec<&str>) -> Vec<&'a str> {
    let base = if a.len() < b.len() { b } else { a };
    let target = if a.len() > b.len() { b } else { a };
    let mut result: Vec<&str> = Vec::new();
    for value in base {
        if !target.contains(value) {
            result.push(value);
        }
    }
    result
}

