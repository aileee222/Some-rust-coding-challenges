fn is_peak(list: &[i32], i: usize) -> bool {
    if list.len() < 3 || i == 0 || i == list.len()-1 { return false; }
    
    for a in 0..i { if list[a] >= list[a+1] { return false; } }
    for a in i..list.len()-1 { if list[a] <= list[a+1] { return false; } }

    return true;
}

fn is_mountain(list: &[i32]) -> bool {
    let mut peak_count = 0;
    for i in 0..list.len() {
        if is_peak(list, i) { peak_count += 1; }
    }
    if peak_count == 1 { return true; }
    else { return false; }
}

fn is_trough(list: &[i32], i: usize) -> bool {
    if list.len() < 3 || i == 0 || i == list.len()-1 { return false; }
    
    for a in 0..i { if list[a] <= list[a+1] { return false; } }
    for a in i..list.len()-1 { if list[a] >= list[a+1] { return false; } }

    return true;
}

fn is_valley(list: &[i32]) -> bool {
    let mut trough_count = 0;
    for i in 0..list.len() {
        if is_trough(list, i) { trough_count += 1; }
    }
    if trough_count == 1 { return true; }
    else { return false; }
}

fn LandscapeType(list: &[i32]) -> &str {
    if is_valley(list) { return "valley"; }
    else if is_mountain(list) { return "mountain"; }
    else { return "neither"; }
}

fn main() {
    println!("{}", LandscapeType(&[3, 4, 5, 4, 3]));
    println!("{}", LandscapeType(&[9, 7, 3, 1, 2, 4]));
    println!("{}", LandscapeType(&[9, 8, 9]));
    println!("{}", LandscapeType(&[9, 8, 9, 8]) );
}