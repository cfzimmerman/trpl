fn main() {
    let inputs: Vec<i32> = vec![0, 5, 7, 1, 2, 2, 0, 9, -11];
    // let inputs: Vec<i32> = vec![5, 2, 3, 1];
    // let inputs: Vec<i32> = vec![0, 7, -3, 5, 9, 4, 4, 3];
    let sorted: Result<Vec<i32>, Box<dyn std::error::Error>> = sort::quicksort(inputs);
    println!("{:?}", sorted)
}

pub mod sort {
    use rand::Rng;
    use std::error;

    pub fn quicksort(nums: Vec<i32>) -> Result<Vec<i32>, Box<dyn error::Error>> {
        let last_index: usize = nums.len() - 1;
        run_quicksort(nums, 0, last_index)
    }

    fn run_quicksort(
        nums: Vec<i32>,
        left: usize,
        right: usize,
    ) -> Result<Vec<i32>, Box<dyn error::Error>> {
        if left >= right {
            return Ok(nums);
        }
        let (nums, pivot_ind) = partition(nums, left, right)?;
        let nums = match pivot_ind {
            0 => nums,
            _ => run_quicksort(nums, left, pivot_ind - 1)?,
        };
        run_quicksort(nums, pivot_ind + 1, right)
    }

    fn rand_ind(min: usize, max: usize) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(min, max)
    }

    fn partition(
        mut nums: Vec<i32>,
        left_start: usize,
        right_start: usize,
    ) -> Result<(Vec<i32>, usize), Box<dyn error::Error>> {
        let mut left = left_start;
        let mut right = right_start;

        nums.swap(left_start, rand_ind(left_start, right_start));
        let pivot = nums[left_start];
        left += 1;

        while left <= right {
            while left <= right && nums[left] < pivot {
                left += 1;
            }
            while left <= right && nums[right] > pivot {
                right -= 1;
            }
            if left <= right {
                nums.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        nums.swap(left_start, right);
        Ok((nums, right))
    }
}
