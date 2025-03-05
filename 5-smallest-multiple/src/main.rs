fn main() {
    println!("{}", find_smallest_multiple());
}

fn find_smallest_multiple() -> i64 {
    let mut i: i64 = 1;
    while check_multiples(i, 20) && i < 10000000000 {
        i += 1;
    }
    return i;
}

fn check_multiples(num: i64, biggest_mult: i64) -> bool {
    for n in 2..=biggest_mult {
        if num % n != 0 {
            return true;
        }
    }
    return false;
}
