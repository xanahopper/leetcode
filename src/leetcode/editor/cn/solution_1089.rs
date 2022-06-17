//给你一个长度固定的整数数组 arr，请你将该数组中出现的每个零都复写一遍，并将其余的元素向右平移。 
//
// 注意：请不要在超过该数组长度的位置写入元素。 
//
// 要求：请对输入的数组 就地 进行上述修改，不要从函数返回任何东西。 
//
// 
//
// 示例 1： 
//
// 输入：[1,0,2,3,0,4,5,0]
//输出：null
//解释：调用函数后，输入的数组将被修改为：[1,0,0,2,3,0,0,4]
// 
//
// 示例 2： 
//
// 输入：[1,2,3]
//输出：null
//解释：调用函数后，输入的数组将被修改为：[1,2,3]
// 
//
// 
//
// 提示： 
//
// 
// 1 <= arr.length <= 10000 
// 0 <= arr[i] <= 9 
// 
// Related Topics 数组 双指针 👍 200 👎 0


use crate::leetcode::Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let n = arr.len();
        let count = arr.iter().filter(|x| **x == 0).count();
        (0..n).rev().fold(count, |c, i| {
            let c = if arr[i] == 0 {
                c - 1
            } else {
                c
            };
            if i + c < n {
                arr[i + c] = arr[i];
                if arr[i] == 0 && i + c + 1 < n {
                    arr[i + c + 1] = 0
                }
            }
            c
        });
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[test]
fn solution_1089_test() {
    let mut arr1 = vec![1, 0, 2, 3, 0, 4, 5, 0];
    Solution::duplicate_zeros(&mut arr1);
    assert_eq!(arr1, vec![1, 0, 0, 2, 3, 0, 0, 4]);

    let mut arr2 = vec![1, 2, 3];
    Solution::duplicate_zeros(&mut arr2);
    assert_eq!(arr2, vec![1, 2, 3]);
}