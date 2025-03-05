use std::convert::TryInto;

fn main() {
    println!("{}", calculate_primes(10001));
}

fn calculate_primes(n_prime:i64) -> i64 {
    let mut prime_list:Vec<i64> = vec![];
    let mut i:i64 = 0;
    while prime_list.len() <= (n_prime).try_into().unwrap() {
        if is_prime(i) {
            prime_list.push(i);
        }
        i += 1;
    }
    return prime_list[prime_list.len()-1];
}

fn is_prime(num: i64) -> bool {
    let _root: i64 = (num as f64).sqrt() as i64 + 1;
    if num % 2 == 0 && num != 2 {
        return false;
    } else {
        for i in 3..=_root {
            if num % i == 0 {
                return false;
            }
        }
    }
    return true;
}
