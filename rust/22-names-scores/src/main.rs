use std::fs;

fn main() {
    // init total_score value
    let mut total_score: i32 = 0;
    // read txt file, push contents into a vec of name strings
    let file_path = "./assets/names.txt";
    let mut contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    // remove first and last quotes (forgor)
    contents.pop();
    contents.remove(0);
    // creat vec of names
    let mut name_vec: Vec<&str>;
    name_vec = contents.split("\",\"").collect();
    //println!("{}, {}", name_vec[0], name_vec[name_vec.len() - 1]);
    // later we will sort ourselves, for now easy builtin sort
    name_vec.sort();

    /*
    // tally up score = sum(chars in name)*position in list
    let index = 0;
    let string = name_vec[index];
    println!("{}", string);
    */
    let string_test: &str = "abcdefghijklmnopqrstuvwxyz";
    
    let mut char_value_vec:Vec<i32> = vec![];
    string_test.chars().for_each(|c| char_value_vec.push(c.to_ascii_lowercase() as i32 - 96));
    println!("{:?}", char_value_vec);
    for i in 0..name_vec.len() {
        let mut char_sum: i32 = 0;
        // get name
        let string = name_vec[i];
        string.chars().for_each(|c| char_sum += c.to_ascii_lowercase() as i32 - 96);
        total_score += char_sum * (i + 1) as i32;
        //println!("{}", char_sum * i as i32);
    }
    println!("{}", total_score);
    //println!("{}", name_vec.len());
}