from collections import defaultdict
from pathlib import Path
from functools import cmp_to_key
from pprint import pprint



def solve(file_path: Path):
    with open(file_path) as f:
        ordering, lists = f.read().split('\n\n')


    ordering = [list(map(int, x.split("|"))) for x in ordering.strip().split('\n')]


    lists = [list(map(int, x.split(","))) for x in lists.strip().split('\n')]
    print("lists:")
    pprint(lists)

    predecessors = defaultdict(set)

    for o in ordering:
        predecessors[o[1]].add(o[0])

    # Define a custom comparison function
    def custom_cmp(x, y):
       if y in predecessors.get(x, set()):
           return 1
       return -1


    custom_key = cmp_to_key(custom_cmp)

    new_lists = [sorted(l, key=custom_key) for l in lists]
    print("sorted lists:")
    pprint(new_lists)


    changed_lists = [sorted_list for original, sorted_list in zip(lists, new_lists) if original != sorted_list]

    middle_elements = [ l[len(l) // 2] for l in changed_lists]
    print(middle_elements)
    middle_sum = sum(middle_elements)
    print(middle_sum)



if __name__ == '__main__':
    #input_file = Path.cwd() / 'input_test.txt'
    input_file = Path.cwd() / 'input.txt'

    solve(input_file)
