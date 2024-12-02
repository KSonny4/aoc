from pathlib import Path

def count_diff(data: list[int]) -> int:
    differences = [data[i + 1] - data[i] for i in range(len(data) - 1)]

    # Check if all differences are in the allowed set
    valid_increasing = all(diff in {1, 2, 3} for diff in differences)
    valid_decreasing = all(diff in {-1, -2, -3} for diff in differences)

    return valid_increasing or valid_decreasing

def generate_deleted_options(data):
    return [data[:i] + data[i+1:] for i in range(len(data))]



def solve(file_path: Path):
    with open(file_path) as f:
        data = [line.strip() for line in f.readlines()]

    data = [list(map(int, x.split(" "))) for x in data]

    counter = 0
    for d in data:
        deleted_options: list[list[int]] = generate_deleted_options(d)
        counter += int(any(count_diff(de) for de in deleted_options))

    print(counter)
if __name__ == '__main__':
    input_file = Path.cwd()  / 'input.txt'
    solve(input_file)
