from pathlib import Path
import re


def solve_reg(data:str):

    # Regex pattern to match all valid "mul(x, y)" calls
    pattern = r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)|do\(\)|don\'t\(\)"


    # Find all valid "mul(x, y)" matches
    matches = re.findall(pattern, data)

    result = sum((int(x) * int(y) for (x, y) in matches))
    # Print the matches
    #print(result)  # Outputs: [('2', '4'), ('11', '8'), ('8', '5'), ('2', '4'), ('5', '5')]
    return result

def solve(file_path: Path):
    with open(file_path) as f:
        data = [line.strip() for line in f.readlines()]

    counter = 0
    for d in data:
        counter += solve_reg(d)
    print(counter)
if __name__ == '__main__':
    input_file = Path.cwd()  / 'input_test.txt'
    solve(input_file)
