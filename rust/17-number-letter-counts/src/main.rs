use std::iter::successors;
fn main() {
    count_letters_to_1000();
}

/*
// for digits, 10s, etc it is harder. when the numbers get bigger there is less work to do
fn count_letters(input_num: i32) -> i32 {
    // final result
    let mut count = 0;
    let mut num = input_num;
    // define letter counts
    let hundred_letter: i32 = 7;
    let thousand_letter: i32 = 8;
    let twenty_ninety: Vec<i32> = vec![0, 0, 6, 6, 6, 5, 5, 7, 5, 6]; // 20-90, letter counts for the words
    let ten_ninteen: Vec<i32> = vec![3, 6, 6, 8, 8, 7, 7, 9, 8, 8]; // 10-19, letter counts for the words
    let zero_nine: Vec<i32> = vec![4, 3, 3, 4, 4, 4, 3, 5, 5, 4]; // 0-9, letter counts for the words

    // generate array of digits
    let mut num_arr: Vec<i32> = vec![];
    let num_len = successors(Some(num), |&num| (num >= 10).then(|| num / 10)).count();
    for _ in 1..=num_len {
        num_arr.push(num % 10);
        num /= 10;
    }

    match num_len as i32 {
        1 => count += zero_nine[num_arr[0] as usize],
        2 => {
            if num_arr[0] == 1 {
                count += ten_ninteen[num_arr[1] as usize];
            } else { 
                count += twenty_ninety[num_arr[0] as usize];
                count += zero_nine[num_arr[1] as usize];
            }
        }
        3 => {
            count += hundred_letter;
            count += zero_nine[num_arr[0] as usize];
            if num_arr[1] == 1 {
                count += ten_ninteen[num_arr[2] as usize]
            } else {
                count += twenty_ninety[num_arr[1] as usize];
                count += zero_nine[num_arr[2] as usize];
            }
        }
        4 => count += thousand_letter,
        _ => (),
        }
    return count;
}
*/

fn count_letters(num:i64) -> i64 {
    // define number letter counts
    let mut count:i64 = 0;
    let mut _num:i64 = num;
    let hundred:i64 = 7;
    let thousand:i64 = 8;
    let one_nineteen: Vec<i64> = vec![0, 3, 3, 4, 4, 4, 3, 5, 5, 4, /*all digits done*/ 3, 6, 6, 8, 8, 7, 7, 9, 7, 8]; // index 0 is 0, every index after that is its letter count
    let twenty_ninety:Vec <i64> = vec![0, 0, /*empty for index 0 and 1*/ 6, 6, 6, 5, 5, 7, 6, 6]; // index 2 is 20;

    // generate vec of digits that make up num
    let mut num_arr: Vec<i64> = vec![];
    let num_len = successors(Some(num), |&num| (num >= 10).then(|| num / 10)).count();
    for _ in 1..=num_len {
        num_arr.push(num % 10);
        _num /= 10;
    }

    match num_len as i64 {
        1 => {
            count += one_nineteen[num_arr[0] as usize];
        },
        2 => {
            count += if num_arr[0] == 1 {one_nineteen[(num_arr[0] + num_arr[1]) as usize]} else {twenty_ninety[num_arr[0] as usize] + one_nineteen[num_arr[1] as usize]};
        },
        3 => {
            // case matching for one to nineteen
            count += if num_arr[1] == 1 {one_nineteen[(num_arr[1] + num_arr[2]) as usize]} else {twenty_ninety[num_arr[1] as usize] + one_nineteen[num_arr[2] as usize]};
            count += 3; // "and" keyword
            count += hundred;
            count += one_nineteen[num_arr[0] as usize];


        },
        4 => {
            count += one_nineteen[1] + thousand;
        },
        _ => panic!("this function was not made for arrays of the wrong size!!!"),
    }
    return count;
}

fn count_letters_to_1000() {
    let mut sum = 0;
    for n in 1..=1000 {
        sum += count_letters(n);
    }
    println!("{}", sum)
}
