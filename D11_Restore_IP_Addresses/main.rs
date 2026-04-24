fn is_valid_part(s: &str) -> bool {
    if s.is_empty() || s.len() > 3 {
        return false;
    }

    if s.starts_with('0') && s.len() > 1 {
        return false;
    }

    match s.parse::<u32>() {
        Ok(n) if n <= 255 => true,
        _ => false,
    }
}

fn backtrack(s: &str, start: usize, parts: &mut Vec<String>, result: &mut Vec<String>) {
    if parts.len() == 4 {
        if start == s.len() {
            result.push(parts.join("."));
        }
        return;
    }

    for len in 1..=3 {
        if start + len > s.len() {
            break;
        }

        let part = &s[start..start + len];

        if is_valid_part(part) {
            parts.push(part.to_string());
            backtrack(s, start + len, parts, result);
            parts.pop();
        }
    }
}

fn restore_ip_addresses(s: String) -> Vec<String> {
    let mut result = Vec::new();
    let mut parts = Vec::new();

    backtrack(&s, 0, &mut parts, &mut result);

    result
}

fn main() {
    let s = "101023".to_string();
    let res = restore_ip_addresses(s);
    dbg!(res);
}