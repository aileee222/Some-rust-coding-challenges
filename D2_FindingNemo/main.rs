fn findNemo(line: &String) {
    let mut entries = line.split_whitespace();
    let mut i = 1;
    while let Some(entry) = entries.next() {
        if entry.contains("Nemo") { println!("I found Nemo at {i}!"); }
        i = i+1;
    }
}

fn main() {
    let line = String::from("I am finding Nemo !");
    let line2 = String::from("Nemo is me");
    let line3 = String::from("I Nemo am");

    findNemo(&line);
    findNemo(&line2);
    findNemo(&line3);
}