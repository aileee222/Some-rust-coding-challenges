fn invert(list: &mut Vec<u8>) {
    for item in list.iter_mut() {
        if *item == 0u8 { *item = 1u8 } else { *item = 0u8 };
    }
}

fn freedPrisoners(list: &mut Vec<u8>) -> u8 {
    let mut res: u8 = 0;

    if list.len() < 1 || list[0] == 0u8 { return 0u8; }

    for i in 0..list.len() {
        if list[i] == 1u8 {
            invert(list);
            res += 1;
        }
    }
    res
}

fn main() {
    let mut prisoner_list: Vec<u8> = Vec::from([1, 1, 0, 0, 0, 1, 0]);
    let mut prisoner_list2: Vec<u8> = Vec::from([1, 1, 1]);
    let mut prisoner_list3: Vec<u8> = Vec::from([0, 0, 0]);
    let mut prisoner_list4: Vec<u8> = Vec::from([0, 1, 1, 1]);

    println!("{}", freedPrisoners(&mut prisoner_list));
    println!("{}", freedPrisoners(&mut prisoner_list2));
    println!("{}", freedPrisoners(&mut prisoner_list3));
    println!("{}", freedPrisoners(&mut prisoner_list4));
}