fn main() {
    iterate_seq();
}



fn iterate_seq() {
    let mut longest_chain_num:i64 = 0;
    let mut vec_len:i64 = 0;
    for n in 1..1_000_000 {
        let temp_len = generate_collatz_seq(n);
        if vec_len < temp_len {
            vec_len = temp_len;
            longest_chain_num = n;
        }
    }
    println!("{}", longest_chain_num);
}

fn generate_collatz_seq(num:i64) -> i64 {
    let mut vec: Vec<i64> = vec![];
    let mut n:i64 = num;
    vec.push(n);
    while n != 1 {
        n = if n % 2 == 0 {n / 2} else {3*n + 1};
        vec.push(n);
    }
    return vec.len() as i64;
}