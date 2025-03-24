fn main() {
    // this needs to be composed of only digits
    const STRING: &str = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";

    println!("{:?}", find_largest_product(str_to_digit_vec(STRING)));
}

// converts a string into a vector of digits.
fn str_to_digit_vec(string: &str) -> Vec<i64> {
    // convert string into array of letters as &str type, not char type
    let mut string_arr: Vec<&str> = string.split("").collect();
    // remove whitespace at index 0 and -1 (end of vector)
    string_arr.pop();
    string_arr.drain(0..1);
    // convert strings to digits and return digit array
    // solution to convert &str to i64: https://hatchjs.com/string-to-int-rust/
    let digit_arr = string_arr.into_iter().map(|s| s.parse().unwrap()).collect();
    return digit_arr;
}

fn find_largest_product(vec: Vec<i64>) -> i64 {
    // set result to 1, being multiplication neutral number
    let mut result: i64 = 1;
    // calculate iter length. we want to not access the vector beyond its final index, so stop at len - 12
    let _iter = vec.len() - 12;
    // iterate the iter
    for n in 0.._iter {
        // create temp result to compare to final
        let mut _temp_result = 1;
        // this lets us get to the 12 numbers after the number at index n
        for m in 0..13 {
            // multiply temp result with the new number
            _temp_result *= &vec[n + m];
        }
        // set result to be temp result if its bigger to get max value
        result = if _temp_result > result {
            _temp_result
        } else {
            result
        };
    }
    return result;
}
