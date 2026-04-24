use std::collections::LinkedList;

fn main() {
    let mut letters: Vec<(char, char, char, char)> = Vec::new();
    
    letters.push(('a', 'b', 'c', '\0')); // 0 ==> 2
    letters.push(('d', 'e', 'f', '\0'));
    letters.push(('g', 'h', 'i', '\0'));
    letters.push(('j', 'k', 'l', '\0'));
    letters.push(('m', 'n', 'o', '\0'));
    letters.push(('p', 'q', 'r', 's'));
    letters.push(('t', 'u', 'v', '\0'));
    letters.push(('w', 'x', 'y', 'z'));

    let mut list: LinkedList<(char, char)> = LinkedList::new();

    let digits = "";
    let chars: Vec<char> = digits.chars().collect();

    if chars.is_empty() { println!("[]"); }
    else if chars.len() == 1 { 
        let a: usize = chars[0].to_digit(10).unwrap() as usize;
        let arr_a = [letters[a - 2].0, letters[a - 2].1, letters[a - 2].2];
        dbg!(&arr_a);
    }
    else {
        for i in 0..chars.len()-1 {
            let a: usize = chars[i].to_digit(10).unwrap() as usize;
    
            for j in i+1..chars.len() {
                let b: usize = chars[j].to_digit(10).unwrap() as usize;
                let arr_a = [letters[a - 2].0, letters[a - 2].1, letters[a - 2].2];
                let arr_b = [letters[b - 2].0, letters[b - 2].1, letters[b - 2].2];

                for i_l in 0..3 {
                    for j_l in 0..3 {
                        if arr_a[i_l] != '\0' && arr_b[j_l] != '\0' {
                            if !list.contains(&(arr_a[i_l], arr_b[j_l])) { list.push_back((arr_a[i_l], arr_b[j_l])); }
                        }
                    }
                }
            }
        }
        dbg!(&list);
    }
}