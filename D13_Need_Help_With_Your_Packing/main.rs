fn tri_a_bulle(list: &mut [u8]) {
    let n = list.len();
    if n <= 1 { return; }

    for i in (1..list.len()).rev() {
        for j in 0..i {
            if list[j+1] > list[j] {
                let tmp: u8 = list[j+1];
                list[j+1] = list[j];
                list[j] = tmp;
            }
        }
    }
}

fn get_bag_weight(bag: &Vec<u8>) -> u8 {
    let mut w: u8 = 0;
    if bag.is_empty() { return w; }
    for b in bag.iter() {
        w += *b;
    }
    return w;
}

fn CanFit_FFD(list: &mut [u8], b: usize) -> bool {
    let mut bags: Vec<Vec<u8>> = Vec::new();

    tri_a_bulle(list);

    for i in 0..list.len() {
        let mut pushed: bool = false;
        for bag in bags.iter_mut() {
            if 10 - get_bag_weight(bag) >= list[i] { 
                bag.push(list[i]); 
                pushed = true;
                break;
            } 
        }
        if !pushed { bags.push(vec![list[i]]); }
    }
    dbg!(&bags);
    if bags.len() == b { return true; }
    else { return false; }
}

fn CanFit_GFD(list: &mut [u8], b: u8) -> bool {
    let mut bags: Vec<Vec<u8>> = Vec::new();

    let mut bag_rem = b;

    for i in 0..list.len() {
        if list[i] > bag_rem        
    }
    dbg!(&bags);
    if bags.len() == b { return true; }
    else { return false; }
}

fn main() {
    println!("{}", if CanFit_FFD(&mut [2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2], 4) { "true" } else { "false" });
    println!("{}", if CanFit_FFD(&mut [2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2 ], 4) { "true" } else { "false" });

    println!("{}", if CanFit_BFD(&mut [2, 1, 2, 5, 4, 3, 6, 1, 1, 9, 3, 2], 4) { "true" } else { "false" });
    println!("{}", if CanFit_BFD(&mut [2, 7, 1, 3, 3, 4, 7, 4, 1, 8, 2 ], 4) { "true" } else { "false" });
}