from pathlib import Path
from pprint import pprint

def matches(x, y, pattern, grid):
    pat_rows, pat_cols = len(pattern), len(pattern[0])

    print(f"Checking pattern at position ({x}, {y}):")
    for row in pattern:
        print("".join(row))

    for i in range(pat_rows):
        for j in range(pat_cols):
            if pattern[i][j] != "." and grid[x + i][y + j] != pattern[i][j]:
                print(f"Mismatch at ({x + i}, {y + j}): Expected {pattern[i][j]}, found {grid[x + i][y + j]}")
                return False
    return True


def generate_patterns():
    patterns = []

    pattern1 = [["M", ".", "S"],
                [".", "A", "."],
                ["M", ".", "S"]]

    pattern2 = [["S", ".", "M"],
                [".", "A", "."],
                ["S", ".", "M"]]

    pattern3 = [["M", ".", "M"],
                [".", "A", "."],
                ["S", ".", "S"]]

    pattern4 = [["S", ".", "S"],
                [".", "A", "."],
                ["M", ".", "M"]]


    patterns.extend([pattern1, pattern2, pattern3, pattern4])

    return patterns


def search_multiple_patterns(grid, patterns):
    rows, cols = len(grid), len(grid[0])
    results = 0
    _matches = []

    for pattern in patterns:
        pat_rows, pat_cols = len(pattern), len(pattern[0])

        for r in range(rows - pat_rows + 1):
            for c in range(cols - pat_cols + 1):
                if matches(r, c, pattern, grid):
                    results += 1
                    # Debug: Mark the match found
                    print(f"Match found at ({r}, {c})")
                    _matches.append(f"Match found at ({r}, {c}), pattern: {pattern}")

    return results, _matches


def solve(file_path: Path):
    with open(file_path) as f:
        grid = [list(line.strip()) for line in f.readlines()]

    print("Grid:")
    for row in grid:
        print("".join(row))

    patterns = generate_patterns()

    match_count,_matches = search_multiple_patterns(grid, patterns)

    print(f"Total X-MAS patterns found: {match_count}")
    #pprint(_matches)


# Entry point
if __name__ == '__main__':
    input_file = Path.cwd() / 'input.txt'
    solve(input_file)
