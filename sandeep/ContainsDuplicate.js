// https://leetcode.com/problems/contains-duplicate/description/

/**
 * @param {number[]} nums
 * @return {boolean}
 */
var containsDuplicate = function (nums) {
  let temp = {};
  for (let i = 0; i < nums.length; i++) {
    if (temp[nums[i]] !== undefined) {
      return true;
    } else {
      temp[nums[i]] = i;
    }
  }
  return false;
};
