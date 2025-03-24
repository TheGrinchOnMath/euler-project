// import standard library

fn main() {
    let num = 600851475143;
    let vec = find_prime_factors(num);
    println!(
        "the largest prime factor of {} is {:?}",
        num,
        vec[vec.len() - 1]
    );
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

fn find_prime_factors(num: i64) -> Vec<i64> {
    let mut list: Vec<i64> = Vec::new();
    if num != 1 {
        for i in 2..=((num as f64).sqrt() as i64 + 1) {
            if is_prime(i) && num % i == 0 {
                list.push(i);

                find_prime_factors(num / i);
            }
        }
    }

    return list;
}
