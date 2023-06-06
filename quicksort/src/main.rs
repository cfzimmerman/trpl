fn main() {
    let inputs: Vec<i32> = vec![0, 7, -3, 5, 9, 4, 4, 3];
    let sorted = sort::quicksort(inputs);
    println!("{:?}", sorted)
}

pub mod sort {
    use std::io;

    pub fn quicksort(nums: Vec<i32>) -> Result<Vec<i32>, io::Error> {
        // The partition's right pointer sometimes goes one below the last element
        // in the array. We reserve an extra index placeholder to prevent exceeding
        // usize bounds in negative number.
        let mut safe_nums = vec![0];
        safe_nums.extend(nums.iter());
        let last_index: usize = safe_nums.len() - 1;
        let mut nums = run_quicksort(safe_nums, 1, last_index)?;
        nums.remove(0);
        Ok(nums)
    }

    fn run_quicksort(nums: Vec<i32>, left: usize, right: usize) -> Result<Vec<i32>, io::Error> {
        if left >= right {
            return Ok(nums);
        }
        let (nums, pivot_ind) = partition(nums, left, right)?;
        let nums = run_quicksort(nums, left, pivot_ind - 1)?;
        run_quicksort(nums, pivot_ind + 1, right)
    }

    fn partition(
        mut nums: Vec<i32>,
        left_start: usize,
        right_start: usize,
    ) -> Result<(Vec<i32>, usize), io::Error> {
        let mut left = left_start;
        let mut right = right_start;

        let pivot_ind = right_start;
        let pivot = nums[right_start];
        right -= 1;

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

        nums.swap(left, pivot_ind);
        Ok((nums, left))
    }
}

// [0, 5, 7, 1, 2, 2, 0, 9, -11]
