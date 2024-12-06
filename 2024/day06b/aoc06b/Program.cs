using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;

enum State
{
    EndOfMap,
    Obstruction,
    Clear,
}

enum Result
{
    NoCycle,
    Cycle
}

enum Direction
{
    Up,
    Down,
    Left,
    Right
}

class Program
{
    static void Main(string[] args)
    {
        var inputFilePath = Path.Combine(
            AppDomain.CurrentDomain.BaseDirectory,
            "../../../input.txt"
        );
        Solve(inputFilePath);
    }

    static State CheckDirection(List<List<char>> data, int x, int y)
    {
        return (x, y) switch
        {
            _ when x < 0 || x >= data.Count => State.EndOfMap,
            _ when y < 0 || y >= data[x].Count => State.EndOfMap,
            _ when data[x][y] == '#' => State.Obstruction,
            _ when data[x][y] == 'O' => State.Obstruction,
            _ when data[x][y] == '.' => State.Clear,
            _ when data[x][y] == 'X' => State.Clear,
            _ => throw new Exception("Invalid state")
        };
    }

    private static Direction Rotate90Degrees(Direction guard)
    {
        return guard switch
        {
            Direction.Up => Direction.Right,
            Direction.Left => Direction.Up,
            Direction.Down => Direction.Left,
            Direction.Right => Direction.Down,
            _ => throw new Exception("Invalid guard")
        };
    }

    static void Solve(string filePath)
    {
        var content = File.ReadAllText(filePath);

        List<List<char>> data = content
            .Split(new[] { "\n" }, StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.ToList())
            .ToList();

        var guardCoordinates = data
            .SelectMany((row, i) =>
                row.Select((c, j) => new { Char = c, Row = i, Col = j }))
            .Where(item => item.Char == '^')
            .Select(item => (item.Row, item.Col))
            .ToList()
            .First();

        // get all "." from data
        var possibleObstacles = data
            .SelectMany((row, i) =>
                row.Select((c, j) => new { Char = c, Row = i, Col = j }))
            .Where(item => item.Char == '.')
            .Select(item => (item.Row, item.Col))
            .ToList();

        // count of possible obs
        Console.WriteLine($"Number of possible obstacles: {possibleObstacles.Count}");

        var obstacles = possibleObstacles
            .AsParallel() // Parallelize the processing
            .Select((possibleObstacle, index) => TestGuard(
                guardCoordinates,
                data.Select(innerList => new List<char>(innerList)).ToList(),
                possibleObstacle,
                index)) // Passing the index as a parameter
            .Count(result => result == Result.Cycle);

        Console.WriteLine($"Number of obstacles: {obstacles}");
    }

    private static Result TestGuard((int Row, int Col) guardCoordinates, List<List<char>> data, (int Row, int Col) possibleObstacle, int index)
    {
        // Add obstacle
        data[possibleObstacle.Row][possibleObstacle.Col] = 'O';
        
        var debug = false;
        (var x, var y) = guardCoordinates;
        var guard = Direction.Up; // Use enum for guard
        data[x][y] = 'X';
        
        
        bool notEndOfTheMap = true;

        var directionDeltas = new Dictionary<Direction, (int dx, int dy)>
        {
            { Direction.Up, (-1, 0) },
            { Direction.Down, (1, 0) },
            { Direction.Left, (0, -1) },
            { Direction.Right, (0, 1) }
        };

        // Start the stopwatch to track time
        Stopwatch stopwatch = Stopwatch.StartNew();

        while (notEndOfTheMap)
        {
            // Check if the 2 seconds have passed
            if (stopwatch.Elapsed.TotalSeconds >= 0.2)
            {
                //Console.WriteLine(index);
                // TODO brutally nasty hack, but it works :DDD
                // If more than 0.2 seconds, we assume the loop is stuck in a cycle
                return Result.Cycle;
            }

            if (debug) { Console.WriteLine($"Guard at ({x}, {y}), facing {guard}"); }
            if (debug) { printMap(data); }

            var (dx, dy) = directionDeltas[guard];
            var nextState = CheckDirection(data, x + dx, y + dy);

            switch (nextState)
            {
                case State.Clear:
                    x += dx;
                    y += dy;
                    data[x][y] = 'X';
                    if (debug) { Console.WriteLine($"Moved to ({x}, {y}), facing {guard}, State: Clear"); }
                    break;

                case State.Obstruction:
                    guard = Rotate90Degrees(guard);
                    if (debug) { Console.WriteLine($"Turned to {guard}, State: Obstruction"); }
                    break;

                case State.EndOfMap:
                    if (debug) { Console.WriteLine("State: EndOfMap - Exiting"); }
                    notEndOfTheMap = false;
                    break;

                default:
                    throw new Exception("Invalid state");
            }
        }

        // If the loop exits within 2 seconds, it's considered NoCycle
        return Result.NoCycle;
    }

    private static void printMap(List<List<char>> data)
    {
        // print map
        Console.WriteLine();
        foreach (var row in data)
        {
            Console.WriteLine(string.Join("", row));
        }
        Console.WriteLine();
    }
}
