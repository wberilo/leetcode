class Solution:
    def searchElements(self, arr, target):
        if not arr:
            return False

        key = math.floor(len(arr) / 2)
        element = arr[key]

        if element == target:
            return True
        elif element > target:
            return self.searchElements(arr[:key], target)
        else:
            return self.searchElements(arr[key+1:], target)

    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        if not matrix:
            return False

        key = math.floor(len(matrix) / 2)
        middle_row = matrix[key]

        if not middle_row:
            return False

        if middle_row[0] <= target <= middle_row[-1]:
            return self.searchElements(middle_row, target)
        elif middle_row[0] > target:
            return self.searchMatrix(matrix[:key], target)
        else:
            return self.searchMatrix(matrix[key+1:], target)