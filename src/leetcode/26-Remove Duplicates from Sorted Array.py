def removeDuplicates(nums) -> int:
  key = 1;
  lastnum = nums[0]
  for num in nums:
    if num > lastnum:
      nums[key] = num;
      lastnum = num;
      key+=1
  print(nums)
  return key

removeDuplicates([1,1,2,2,3])