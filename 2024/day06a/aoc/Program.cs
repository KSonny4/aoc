using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

enum State
{
    EndOfMap,
    Obstruction,
    Clear,
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
            _ when data[x][y] == '.' => State.Clear,
            _ when data[x][y] == 'X' => State.Clear,
            _ => throw new Exception("Invalid state")
        };
    }

    private static string Rotate90Degrees(string guard)
    {
        return guard switch
        {
            "^" => ">",
            "<" => "^",
            "v" => "<",
            ">" => "v",
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
            .ToList()[0];

        (var x, var y) = guardCoordinates;
        var guard = "^";
        data[x][y] = 'X';
        bool notEndOfTheMap = true;
        var directionDeltas = new Dictionary<string, (int dx, int dy)>
        {
            { "^", (-1, 0) },
            { "v", (1, 0) },
            { "<", (0, -1) },
            { ">", (0, 1) }
        };
        
        while (notEndOfTheMap)
        {
            Console.WriteLine($"Guard at ({x}, {y}), facing {guard}");

            var (dx, dy) = directionDeltas[guard];
            var nextState = CheckDirection(data, x + dx, y + dy);

            switch (nextState)
            {
                case State.Clear:
                    x += dx;
                    y += dy;
                    data[x][y] = 'X';
                    Console.WriteLine($"Moved to ({x}, {y}), facing {guard}, State: Clear");
                    break;

                case State.Obstruction:
                    guard = Rotate90Degrees(guard);
                    Console.WriteLine($"Turned to {guard}, State: Obstruction");
                    break;

                case State.EndOfMap:
                    Console.WriteLine("State: EndOfMap - Exiting");
                    notEndOfTheMap = false;
                    break;
                default:
                    throw new Exception("Invalid state");
            }
        }
        var count = data.SelectMany(row => row).Count(c => c == 'X');
        Console.WriteLine($"Number of X: {count}");
    }
}
