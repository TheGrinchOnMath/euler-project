use num_bigint::BigUint;

fn main() {

    let factor_100 = factorial_recursive(BigUint::from(100u32));
    println!("Sum of digits of 100!: {}", big_uint_digit_sum(factor_100));
}

fn factorial_recursive(n: num_bigint::BigUint) -> num_bigint::BigUint {
    let one = num_bigint::BigUint::from(1u8);
    if n == one {
        return one;
    } else {
        return factorial_recursive(n.clone() - one) * n;
    }
}

fn big_uint_digit_sum(n: num_bigint::BigUint) -> u32 {
    let vec = n.to_string().chars().flat_map(|c| c.to_digit(10)).collect::<Vec<u32>>();

    return vec.iter().sum();
}