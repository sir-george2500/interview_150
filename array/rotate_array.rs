impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        //shadowing k
        let k = (k % n as i32) as usize;

        Self::reverse(nums, 0, n - 1);
        Self::reverse(nums, 0, k.saturating_sub(1)); // k-1
        Self::reverse(nums, k, n - 1);
    }

    pub fn reverse(nums: &mut Vec<i32>, start: usize, end: usize) {
        let mut start = start;
        let mut end = end;
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
}
