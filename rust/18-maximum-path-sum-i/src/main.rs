fn main() {
    dfs();
}

// BAD APPROACH, DOESN'T TAKE INTO ACCOUNT CERTAIN OPTIONS.

fn dfs() {
    // create vector of pyramid
    let path: Vec<Vec<u32>> = vec![
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

    // create vector of longest row, to hold the sum for each starting point.
    let mut sum: Vec<u32> = path[path.len() - 1].iter().copied().collect();
    let mut min_sum = u32::MAX;

    // iterate over starting_points
    for start_point in (0..sum.len()).rev() {
        // start recursion
        sum[start_point] = recursive_sum(
            (start_point as usize, path.len() - 1),
            &path,
            sum[start_point],
        );
        min_sum = if sum[start_point] < min_sum {
            sum[start_point]
        } else {
            min_sum
        };
    }
    println!("sum: {:?}", min_sum);
}

// pos: (x, y) where x is horizontal, y vertical. sum_prev is the result from the previous iteration.
fn recursive_sum(pos: (usize, usize), path: &Vec<Vec<u32>>, sum_prev: u32) -> u32 {
    let row_above: Vec<u32> = path[pos.1].iter().cloned().collect();
    // check if topmost row
    if row_above.len() == 1 {
        // return position (0, 0) & the sum. Since the function no longer calls itself it ends the iteration loop.
        return sum_prev + row_above[0];
    }
    // not at the end of the recursion.
    else {
        // stuck to left edge
        if pos.0 == 0 {
            // call the recursive sum function for the row above, starting once again on left edge.
            recursive_sum((0, pos.1 - 1), path, sum_prev + row_above[0])
        }
        // right edge
        else if pos.0 == (row_above.len()) - 1 {
            // call the recursive sum, stuck on right edge. decrement pos.0 to avoid going out of bounds.
            recursive_sum((pos.0 - 1, pos.1 - 1), path, sum_prev + row_above[pos.0])
        }
        // not on an edge
        else {
            // "right" option
            let right = row_above[pos.0 - 1];
            // "left" option
            let left = row_above[pos.0];

            // go left
            if right > left {
                recursive_sum((pos.0, pos.1 - 1), path, sum_prev + left)
            }
            // go right
            else if right < left {
                recursive_sum((pos.0 - 1, pos.1 - 1), path, sum_prev + right)
            }
            // both options are identical in value.
            else {
                // check the options above left and right to see if there is a better path.

                // check left option.
                let option_1 = recursive_sum((pos.0 - 1, pos.1 - 1), path, left);

                let option_2 = recursive_sum((pos.0, pos.1 - 1), path, right);

                // left path smaller, so take it.
                return if option_1 < option_2 {
                    sum_prev + option_1
                } else {
                    sum_prev + option_2
                };
            }
        }
    }
}

/*
description of algorithm.
given: current position in pyramid, sum of previous iteration:
1. check both above cells (above and to the left) [check bounding here]
2. find the smaller above cell
3. update the position to the smaller cell, add it to the sum
4. if the vector for the row above contains one number, terminate algorithm and start from next starting_position.
*/
