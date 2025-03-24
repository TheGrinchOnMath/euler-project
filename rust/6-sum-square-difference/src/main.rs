use std::cmp;

fn main() {
    println!("{:?}", sum_square_difference(100));
}

fn sum_square_difference(size: i64) -> i64 {
    let mut sum_squares: i64 = 0;
    let mut square_sum: i64 = 0;
    for i in 1..=size {
        sum_squares += i * i;
        square_sum += i;
    }
    square_sum *= square_sum;
    return cmp::max(sum_squares, square_sum) - cmp::min(sum_squares, square_sum);
}
