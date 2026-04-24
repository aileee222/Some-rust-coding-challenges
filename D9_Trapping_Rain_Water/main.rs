fn is_trapped_left(id: usize, height: &Vec<u32>) -> bool {

    for i in id..height.len() {
        if height[i] != 0 { return true; }
    }
    return false
}

fn is_trapped_right(id: usize, height: &Vec<u32>) -> bool {
    for i in 0..id {
        if height[i] != 0 { return true; }
    }
    return false
}

fn is_trapped(id: usize, height: &Vec<u32>) -> bool {
    if id == 0 || id == height.len()-1 { return false; }

    return is_trapped_left(id, height) && is_trapped_right(id, height);
}

fn get_max(height: &Vec<u32>) -> usize {
    let mut max: u32 = 0;
    let mut id: usize = 0;
    for (i, h) in height.iter().enumerate() {
        if *h > max { max = *h; id = i; }
    }
    return id;
}

fn minimize_height(height: &mut Vec<u32>) {
    for h in height.iter_mut() { if *h > 0 { *h -= 1; } }
}

fn main() {
    let mut height: Vec<u32> = Vec::from([4,2,0,3,2,5]);
    let mut water_trapped: u32 = 0;
    for i in 0..get_max(&height) {
        for (i, h) in height.iter().enumerate() {
           if *h == 0 && is_trapped(i, &height) { water_trapped += 1; }
        }

        minimize_height(&mut height);
    }
    dbg!(water_trapped);
}