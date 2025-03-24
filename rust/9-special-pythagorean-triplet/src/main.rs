fn main() {
    find_pythagorean_triplet();
}

fn find_pythagorean_triplet() {
    // size of iteration
    let _ceil: i64 = 500;
    for a in 1..=_ceil {
        for b in 1..=_ceil {
            for c in 1..=_ceil {
                // check conditions for a pythagorean triplet: a² + b² = c², a < b < c and a + b + c = 1000 the special condition
                if a * a + b * b == c * c && a < b && b < c && a + b + c == 1000 {
                    println!("{}", a * b * c);
                }
            }
        }
    }
}