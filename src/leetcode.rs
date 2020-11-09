// Input: nums = [1,2,3,4]
// Output: [1,3,6,10]
// Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut next: i32 = 0;
    let mut result: Vec<i32> = Vec::new();
    for i in nums {
        next = next + i;
        result.push(next);
    }
    return result;
}

pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut max_candy: i32 = 0;
    let mut result: Vec<bool> = Vec::new();

    for candy in &candies {
        if *candy > max_candy {
            max_candy = *candy;
        }
    }

    for candy in &candies {
        let new_candy = *candy + extra_candies;
        if new_candy >= max_candy {
            result.push(true);
        } else {
            result.push(false);
        }
    }

    return result;
}

struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {

    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        let rectangle = SubrectangleQueries {
            rectangle: rectangle.clone()
        };
        return rectangle;
    }
    
    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for i in row1..row2+1 {
            for j in col1..col2+1 {
                self.rectangle[i as usize][j as usize] = new_value;
                println!("{}", self.rectangle[i as usize][j as usize]);
            }
        }
    }
    
    fn get_value(&self, row: i32, col: i32) -> i32 {
        return self.rectangle[row as usize][col as usize];
    }
}