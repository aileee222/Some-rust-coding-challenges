fn josephus(soldiers: &mut Vec<usize>, nb_soldier: usize, i: usize) {
    let mut index: usize = i;

    while soldiers.len() > 1 {
        if index <= 0 { index = soldiers.len()-index; }
        else if index >= soldiers.len() { index -= (soldiers.len() + 1); }

        soldiers.remove(index);
        dbg!(soldiers.len());
        dbg!(index);
        dbg!(&soldiers);
        index += (i-1); 
    }
    // dbg!(&soldiers);
}

fn main() {
    let nb_soldier: usize = 41;
    let i: usize = 3;

    let mut soldiers: Vec<usize> = Vec::new();
    for i in 1..nb_soldier+1 {
        soldiers.push(i)
    }
    dbg!(&soldiers);
    josephus(&mut soldiers, nb_soldier, i);
}