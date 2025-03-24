fn main() {
    println!("{}", calculate_lattice(20, 20));
}

fn factorial(num: i64) -> i64 {
    let mut res: i64 = 1;
    for n in 2..=num {
        res *= n;
    }
    return res;
}

fn calculate_lattice(height: i64, width: i64) -> i64 {
    let entity_count: i64 = height + width;
    let entity_count_factorized: i64 = factorial(entity_count);
    let height_factorized: i64 = factorial(height);
    let width_factorized: i64 = factorial(width);
    let res = entity_count_factorized / (height_factorized * width_factorized);
    return res;
}
