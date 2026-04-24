use std::convert::TryInto;

fn progressDays(list: &Vec<i32>, mut id: usize, mut count: i32) -> i32 {
    if list.len() < 2 { return count; }

    for id in 0..list.len()-1 {;
        if list[id+1] > list[id] { 
           count = count+1;
        }
    }
    return count;
}

fn main() {
    let vec = Vec::from([3, 4, 1, 2]);
    let vec2 = Vec::from([10, 11, 12, 9, 10]);
    let vec3 = Vec::from([6, 5, 4, 3, 2, 9]);
    let vec4 = Vec::from([9, 9]);
    
    let mut count: i32 = 0;
    let mut id: usize = 0;

    println!("{}", progressDays(&vec, id, count));
    println!("{}", progressDays(&vec2, id, count));
    println!("{}", progressDays(&vec3, id, count));
    println!("{}", progressDays(&vec4, id, count));
}