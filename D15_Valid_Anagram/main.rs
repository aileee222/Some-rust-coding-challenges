use std::collections::HashMap;

fn main() {
    let mut s_tab: HashMap<char, u8> = HashMap::new();
    let mut t_tab: HashMap<char, u8> = HashMap::new();

    let s: &str = "rat";
    let t: &str = "car";

    let chars_s: Vec<char> = s.chars().collect();
    let chars_t: Vec<char> = t.chars().collect();

    for c in chars_s.iter() {
        if s_tab.contains_key(c) {
            let Some(value) = s_tab.get_mut(c) else { return };
            *value += 1;
        } 
        else {
            s_tab.insert(*c, 0);
        }
    }
    for c in chars_t.iter() {
        if t_tab.contains_key(c) {
            let Some(value) = t_tab.get_mut(c) else { return };
            *value += 1;
        } 
        else {
            let zero: u8 = 0;
            t_tab.insert(*c, zero);
        }
    }

    if s_tab.eq(&t_tab) { println!("equal."); }
    else { println!("not equal."); }
}