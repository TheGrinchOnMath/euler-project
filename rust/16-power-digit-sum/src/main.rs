
use num_bigint::BigUint;
use num_traits::{One, Zero};


fn main() {
    let num = BigUint::one() << 1000;
    digit_sum(num);
}

fn digit_sum(num: BigUint) {
    let mut sum: BigUint = BigUint::from(0 as u8);
    let mut n = num;
    let big10:BigUint = BigUint::from(10 as u16);
    while !n.is_zero() {
        sum += n.clone() % big10.clone();
        n /= big10.clone();
    }

    println!("{}", sum);
}
