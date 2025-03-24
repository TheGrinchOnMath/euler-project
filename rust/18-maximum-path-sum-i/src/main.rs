use std::cmp::max;

fn main() {
    dfs();
}

fn dfs() {
    let path:Vec<Vec<i32>> = vec![
                                    vec![75],
                                  vec![95, 64],
                                vec![17, 47, 82],
                              vec![18, 35, 87, 10],
                            vec![20, 04, 82, 47, 65],
                          vec![19, 01, 23, 75, 03, 34],
                        vec![88, 02, 77, 73, 07, 63, 67],
                      vec![99, 65, 04, 28, 06, 16, 70, 92],
                    vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
                  vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
                vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
              vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
            vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
          vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
    ];

    // we can use the following logic rules: if at very left of array, stay at left (no choice)
    // if at right of array, stick to right,
    // if at middle, take max of both values

    let mut depth = path.len();
    let mut width = path[depth - 1].len();


    let mut current_width;

    // we need to find the largest sum for every "bottom value"
    let mut max_sum = 0;
    let mut max_vec:Vec<i32> = Vec::new();
    
    for starting_width in 0..width {
        let mut current_vec:Vec<i32> = Vec::new();
        let mut current_sum = 0;
        // this stores the point (width wise at which we are
        current_width = starting_width;
        
        // go up the tree
        for current_depth in (0..depth).rev() {


            // we are at left bound of array
            if current_width == 0 {

                current_sum = current_sum + path[current_depth][0];
                current_width = 0;

            // we should be at right bound?
            } else if current_width == path[current_depth].len() - 1 {
                current_sum = current_sum + path[current_depth][current_width];
                current_width -= 1;
            } else {
                let value1 = path[current_depth][current_width];
                let value2 = path[current_depth][current_width - 1];
                if value1 > value2 {
                    current_sum = current_sum + value1;
                    // no need to change the width
                } else if value1 < value2 {
                    current_sum = current_sum + value2;
                    current_width = current_width - 1;
                }
            }
            if current_sum > max_sum {
                max_sum = current_sum;
            }
            println!("{} ", max_sum);
            
        }
    }
}
