from pathlib import Path

def solve(file_path: Path):
    with open(file_path) as f:
        data = [line.strip() for line in f.readlines()]

    data = [x.split("   ") for x in data]

    l1 = sorted([int(x[0]) for x in data])
    l2 = sorted([int(x[1]) for x in data])

    distances = []
    for a,b in zip (l1,l2):
        distances.append(abs(a-b))
    print(sum(distances))
# Entry point
if __name__ == '__main__':
    input_file = Path.cwd() / 'day01a' / 'input.txt'
    solve(input_file)
