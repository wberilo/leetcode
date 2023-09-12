def groupThePeople(groupSizes: List[int]) -> List[List[int]]:
    groups = {}
    for index, key in enumerate(groupSizes):
        if key in groups:
            groups[key].append(index)
        else:
            groups[key] = [index]

    result = []
    for key, group in groups.items():
        num_groups = int(len(group) / key)
        counter = int(0)
        for count in range(num_groups):
            groupToAdd = []
            for keyelement in range(key):
                groupToAdd.append(group[counter])
                counter = counter + 1

            result.append(groupToAdd)
    return result
