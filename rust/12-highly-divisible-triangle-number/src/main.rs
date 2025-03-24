fn main() {
    println!("{}", find_highly_divisible_triangle_number());
}

fn find_divisors(num: i64) -> Vec<i64> {
    let root = (num as f64).sqrt() as i64 + 1;
    let mut vec: Vec<i64> = vec![];
    for n in 1..=root {
        if num % n == 0 {
            vec.push(n);
            vec.push(num / n);
        }
    }
    return vec;
}

fn find_highly_divisible_triangle_number() -> i64 {
    let mut seq_tracker: i64 = 1;
    let mut vec_tracker: i64 = 0;
    let mut res: i64 = 0;
    while vec_tracker < 500 {
        let triangle_number = generate_triangle_number(seq_tracker);
        let temp_vec = find_divisors(triangle_number);
        if vec_tracker < temp_vec.len().try_into().unwrap() {
            res = triangle_number;
            vec_tracker = temp_vec.len().try_into().unwrap();
        }
        seq_tracker += 1;
    }
    return res;
}

fn generate_triangle_number(n_seq: i64) -> i64 {
    return n_seq * (n_seq + 1) / 2;
}
