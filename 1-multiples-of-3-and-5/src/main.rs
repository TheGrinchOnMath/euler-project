fn main() {
    println!("{}", multiples_of_3_5(1, 999).to_string());
}

fn multiples_of_3_5(start: i32, end: i32) -> i32 {
    // the mut keyword lets the variable be mutated, or assigned new data.
    // default is not mutable
    let mut sum: i32 = 0;

    // iterate through the variables passed to the function at call
    for n in start..=end {
        // if n is a multiple of 15
        if n % 15 == 0 {
            // add n to sum
            sum += n;
        // idem
        } else if n % 5 == 0 {
            sum += n;
        // ibidem
        } else if n % 3 == 0 {
            sum += n;
        // skip this iteration of the loop
        } else {
            continue;
        }
    }
    return sum;
}
