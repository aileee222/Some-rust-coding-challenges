use std::collections::HashMap;

fn karacas_enc(word: &str) {
    let mut hashmap = HashMap::new();
    hashmap.insert('a', '0'); 
    hashmap.insert('e', '1'); 
    hashmap.insert('i', '2'); 
    hashmap.insert('o', '2'); 
    hashmap.insert('u', '3'); 

    let word_rev = word.chars().rev();
    let mut word_list: Vec<char> = word_rev.into_iter().collect::<Vec<char>>();

    for letter in word_list.iter_mut() {
        if hashmap.contains_key(letter) { 
            let Some(value) = hashmap.get_mut(letter) else { return };
            *letter = *value;
        } 
    }
    word_list.push('a');
    word_list.push('c');
    word_list.push('a');

    dbg!(word_list);
    return;
}

fn main() {
    karacas_enc("apple");
    karacas_enc("karaca");
    karacas_enc("burak");
    karacas_enc("alpaca");
}