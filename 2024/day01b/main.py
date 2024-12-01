from pathlib import Path
from collections import  Counter
def solve(file_path: Path):
    with open(file_path) as f:
        data = [line.strip() for line in f.readlines()]

    data = [x.split("   ") for x in data]

    l1 = sorted([int(x[0]) for x in data])
    l2 = sorted([int(x[1]) for x in data])

    c = Counter(l2)


    distances = []
    for a in l1:
        num = c.get(a, 0)
        distances.append(a*num)

    print(sum(distances))
# Entry point
if __name__ == '__main__':
    input_file = Path.cwd()  / 'input.txt'
    solve(input_file)
