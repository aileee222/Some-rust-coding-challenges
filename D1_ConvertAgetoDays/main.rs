use std::io;

fn calcAge(age: &i32) -> i32 {
    let day_in_year = 365;
    return age * 365;
}

fn main() {
    let mut age_int: i32;
    let mut is_ok = false;
    println!("Entrez un age: ");

    loop {
        let mut age = String::new(); 
        io::stdin().read_line(&mut age).expect("Failed");

        age_int = match age.trim().parse::<i32>() {
            Ok(nb) => {
                is_ok = true;
                nb
            }
            Err(_) => continue,
        };
        if is_ok { break; }
    }
    let day_in_year = calcAge(&age_int);
    println!("{}.", day_in_year);
}