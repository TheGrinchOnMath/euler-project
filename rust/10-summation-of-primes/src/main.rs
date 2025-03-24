
fn main() {
    sieve_of_erastothenes(2_000_000);
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

fn sieve_of_erastothenes(ceil:i64) {
    let mut sum:i64 = 0;
    for n in 2..ceil {
        sum += if is_prime(n) {n} else {0};
    }
    println!("{}", sum);
}