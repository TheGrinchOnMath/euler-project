use std::cmp::*;

fn main() {
    println!("{:?}", amicable_numbers(1, 10_000));
}

// this works, dont touch
fn divisor_sum(num: i32) -> i32 {
    // init vec to store divisors
    let mut divisors: Vec<i32> = vec![1];
    // calc root (reduces n operations)
    let root = (num as f32).sqrt() as i32 + 1;
    // iter from 2 to root
    // (since divisors come in pairs, with the integer root of the number being a solo pair)
    for n in 2..=root {
        // if n is a divisor of n, add itself and its pair to the divisors vec
        if num % n == 0 {
            divisors.push(n);
            divisors.push(num / n);
        }
    }
    // sort vec, remove duplicates
    divisors.sort();
    divisors.dedup();
    // return sum of all integers in vec
    return divisors.iter().sum();
}

fn amicable_numbers(floor: i32, ceil: i32) -> i32 {
    // init vec to store amicable number pairs
    let mut amicable_vec: Vec<(i32, i32)> = vec![];
    // init result var
    let mut res = 0;
    for n in floor..=ceil {
        // get sum of divisors of n
        let m = divisor_sum(n);
        // first check if the number is amicable to itself (primes and some numbers), skip that
        if m != n {
            // get   sum of divisors of m, check if n and m are amicable
            let m_div = divisor_sum(m);
            if n == m_div {
                // put n and m in ascending size order (for removing duplicates later)
                let smaller = min(n, m);
                let bigger = max(n, m);
                
                // add to vec
                amicable_vec.push((smaller, bigger));
            }
        }
    }
    // sort vector
    amicable_vec.sort();
    // remove duplicates
    amicable_vec.dedup();
    // add all the numbers to the result
    amicable_vec.iter().for_each(|x| res += x.0 + x.1);
    // return result
    res
}
