fn main() {
    println!("{}", fibonacci_even_numbers(4000000).to_string());
}

fn fibonacci_even_numbers(cap:i32) -> i32 {
    let mut sum = 0;
    let target = cap;
    let mut first_term = 0;
    let mut second_term = 1;
    while second_term < target {
        let temp_term = first_term + second_term;
        first_term = second_term;
        second_term = temp_term;
        if second_term % 2 == 0 {
            sum += second_term;
        }
    }
    return sum;
}