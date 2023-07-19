function twoSum(nums: number[], target: number): number[] {
    let hash: { [key: number]: number } = {};
    for (let index = 0; index < nums.length; index++) {
        let diff = target - nums[index];
        if (nums[diff] !== undefined) {
            return [hash[diff], index];
        }
        else {
            hash[nums[index]] = index;
        }
    }
    return [0,0];
};

twoSum([3,3],6);
