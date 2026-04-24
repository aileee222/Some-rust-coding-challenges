fn SockPairs(line: &str) -> i32 {
    let chars: Vec<char> = line.chars().collect();
    let mut list: Vec<(char, char)> = Vec::new();

    for i in 0..chars.len() {
        let mut found: bool = false;
        for j in 0..list.len() {
            if list[j].0.eq(&chars[i]) && list[j].1.eq(&'\0') { 
                list[j].1 = chars[i];
                found = true;
                break;
            }
        }
        if !found { list.push((chars[i], '\0')); }
    }
    let mut count: i32 = 0;
    for j in 0..list.len() {
        if list[j].1 == '\0' { return count; }
        count += 1;
    }
    return count;
}

fn main() {
    let line = "AA";
    let line2 = "ABABC";
    let line3 = "CABBACCC";
    
    println!("{}", SockPairs(&line));
    println!("{}", SockPairs(&line2));
    println!("{}", SockPairs(&line3));
}