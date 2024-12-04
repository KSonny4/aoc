from pathlib import Path

def find_word_positions(grid, word):
    rows, cols = len(grid), len(grid[0])
    word_len = len(word)
    matches = []

    def find_positions(x, y, dx, dy, target):
        positions = []
        for i in range(len(target)):
            nx, ny = x + i * dx, y + i * dy
            if not (0 <= nx < rows and 0 <= ny < cols) or grid[nx][ny] != target[i]:
                return []
            positions.append((nx, ny))
        return positions

    # Search all directions
    def search_and_record(target):
        for r in range(rows):
            for c in range(cols):
                # Diagonals
                for dx, dy in [(1, 1), (1, -1)]:
                    positions = find_positions(r, c, dx, dy, target)
                    if positions:
                        matches.append(positions)
                # Horizontals and Verticals
                for dx, dy in [(0, 1), (1, 0)]:
                    positions = find_positions(r, c, dx, dy, target)
                    if positions:
                        matches.append(positions)

    # Search for the word and its reverse
    search_and_record(word)
    search_and_record(word[::-1])

    return matches


def visualize_grid(grid, matches):
    visual_grid = [[' ' for _ in row] for row in grid]

    for match in matches:
        for x, y in match:
            visual_grid[x][y] = grid[x][y]

    return '\n'.join([' '.join(row) for row in visual_grid])

def solve(file_path: Path):
    with open(file_path) as f:
        grid = [list(line.strip()) for line in f.readlines()]
    word = 'XMAS'

    matches = find_word_positions(grid, word)

    print(f"Number of matches: {len(matches)}")
    # Print matches
    print("Matches (list of indices):")
    for match in matches:
        print(match)

    # Visualize the grid with matches
    print("\nVisualized grid:")
    print(visualize_grid(grid, matches))

    print(f"Number of matches: {len(matches)}")

if __name__ == '__main__':
    #input_file = Path.cwd()  / 'input_test.txt'
    input_file = Path.cwd() / 'input.txt'

    solve(input_file)
