fn unique_path_rec(x: usize, y: usize, start_x: usize, start_y: usize, res: &mut Vec<Vec<String>>, path: &mut Vec<String>, found: &mut Vec<Vec<bool>>) {
    // dbg!(&x);
    // dbg!(&y);
    // dbg!(&start_x);
    // dbg!(&start_y);
    if found[x-1][y-1] == true { 
        println!("one path found!");
        res.push((*path.clone()).to_vec()); 
        return;
    }
    if start_x < x-1 && !found[start_x+1][start_y] { 
        found[start_x+1][start_y] = true;
        path.push("down".to_string());
        unique_path_rec(x, y, start_x+1, start_y, res, path, found);
        found[start_x+1][start_y] = false;
        path.pop();
    }
    if start_x > 0 && !found[start_x-1][start_y] { 
        found[start_x-1][start_y] = true;
        path.push("up".to_string());
        unique_path_rec(x, y, start_x-1, start_y, res, path, found);
        found[start_x-1][start_y] = false;
        path.pop();
    }
    if start_y < y-1 && !found[start_x][start_y+1] { 
        found[start_x][start_y+1] = true;
        path.push("right".to_string());
        unique_path_rec(x, y, start_x, start_y+1, res, path, found);
        found[start_x][start_y+1] = false;
        path.pop();
    }
    if start_y > 0 && !found[start_x][start_y-1] { 
        found[start_x][start_y-1] = true;
        path.push("left".to_string());
        unique_path_rec(x, y, start_x, start_y-1, res, path, found);
        found[start_x][start_y-1] = false;
        path.pop();
    }
}

fn find_unique_path(x: usize, y: usize) {
    let mut res: Vec<Vec<String>> = Vec::new();
    let mut path: Vec<String> = Vec::new();

    let mut found = vec![vec![false; y]; x]; 
    found[0][0] = true;

    unique_path_rec(x, y, 0, 0, &mut res, &mut path, &mut found);
    dbg!(res.len());
}

fn main() {
    find_unique_path(3, 3);
}
