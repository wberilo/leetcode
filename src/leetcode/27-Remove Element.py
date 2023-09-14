def removeElement(nums, val) -> int:
  key = 0;
  for num in nums:
    if num != val:
      nums[key] = num;
      key+=1
  print(nums)
  return key

removeElement([1,1,2,2,3], 1)