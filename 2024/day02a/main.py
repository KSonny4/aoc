from pathlib import Path
from collections import  Counter

def count_diff(data: list[int]) -> int:
    inc = {1,2,3}
    dec = {-1,-2,-3}

    dec_or_inc = inc if data[0] - data[1] > 0 else dec

    for i in range(len(data)-1):
        if not data[i] - data[i+1] in dec_or_inc:
            return 0
    return 1

def solve(file_path: Path):
    with open(file_path) as f:
        data = [line.strip() for line in f.readlines()]

    data = [list(map(int, x.split(" "))) for x in data]

    counter = 0
    for d in data:
        counter += count_diff(d)
    print(counter)
if __name__ == '__main__':
    input_file = Path.cwd()  / 'input.txt'
    solve(input_file)
