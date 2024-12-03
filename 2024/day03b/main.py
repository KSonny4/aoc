from enum import Enum
from pathlib import Path
import re

class Operation(Enum):
    DO = 'do'
    DONT = "don't"

def solve_reg(data: str):
    # Regex pattern to match mul(x, y), do(), and don't()
    pattern = r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)|do\(\)|don\'t\(\)"

    matches = re.finditer(pattern, data)

    result = 0

    do_or_don_t = Operation.DO

    for match in matches:
        # Check the matched string directly
        matched_string = match.group(0)

        if 'mul' in matched_string:
            if do_or_don_t == Operation.DO:
                result += int(match.group(1)) * int(match.group(2))
        elif 'do()' == matched_string:
            do_or_don_t = Operation.DO
        elif "don't()" == matched_string:
            do_or_don_t = Operation.DONT

    return result


def solve(file_path: Path):
    with open(file_path) as f:
        data = [line.strip() for line in f.readlines()]

        counter = 0
        d = ''.join(data)
        counter += solve_reg(d)
        print(counter)

if __name__ == '__main__':
    input_file = Path.cwd() / 'input.txt'
    solve(input_file)
