pub fn run() {
    let mut arr = vec![1, 3, 6, 2, 4, 5];
    println!("平方阶（冒泡排序）{}", bubble_sort(&mut arr));
}

fn bubble_sort(nums: &mut [i32]) -> i32 {
    let mut count = 0; // 计数器
                       // 外循环：未排序区间为 [0, i]
    for i in (1..nums.len()).rev() {
        // 内循环：将未排序区间 [0, i] 中的最大元素交换至该区间的最右端
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                // 交换 nums[j] 与 nums[j + 1]
                let tmp = nums[j];
                nums[j] = nums[j + 1];
                nums[j + 1] = tmp;
                count += 3; // 元素交换包含 3 个单元操作
            }
        }
    }
    count
}
