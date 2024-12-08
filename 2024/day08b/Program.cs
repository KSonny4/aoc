using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

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

    static void Solve(string filePath)
    {
        var data = File.ReadAllText(filePath).Split(new[] { "\n" }, StringSplitOptions.RemoveEmptyEntries).ToList();
        int height = data.Count;
        int width = data.Select(line => line.Length).Max();

        var result = data
            .SelectMany((line, y) => line
                .Select((c, x) => (c, x, y))
                .Where(item => item.c != '.')
            )
            .GroupBy(item => item.c)
            .ToDictionary(
                group => group.Key,
                group => group.Select(item => (item.x, item.y)).ToList()
            );

        var size = (width, height);

        // Collect positions using a normal List
        var positions = new List<(int, int)>();

        // Sequentially process the groups
        foreach (var item in result)
        {
            for (int i = 0; i < item.Value.Count - 1; ++i)
            {
                var posA = item.Value[i];
                for (int j = i + 1; j < item.Value.Count; ++j)
                {
                    var posB = item.Value[j];
                    var diff = (posA.Item1 - posB.Item1, posA.Item2 - posB.Item2);
                    var found = (posA.Item1, posA.Item2);

                    // Add positions for valid ranges
                    CreatePoints(found, size, 0, positions, posA, diff);
                    
                    found = RecomputeValueTuple(posA, diff, -1);
                    
                    CreatePointsNegative(found, size, 0, positions, posA, diff);
                }
            }
        }

        var found2 = positions.Distinct().ToList();
        Console.WriteLine($"Found {found2.Count} positions");
    }

    private static void CreatePointsNegative((int, int) found, (int width, int height) size, int offset, List<(int, int)> positions,
        (int x, int y) posA, (int, int) diff)
    {
        while (IsValidPosition(found, size))
        {
            --offset;
            positions.Add(found);
            found = RecomputeValueTuple(posA, diff, offset);
        }
    }

    private static void CreatePoints((int, int) found, (int width, int height) size, int offset, List<(int, int)> positions,
        (int x, int y) posA, (int, int) diff)
    {
        while (IsValidPosition(found, size))
        {
            ++offset;
            positions.Add(found);
            found = RecomputeValueTuple(posA, diff, offset);
        }
    }

    private static (int, int) RecomputeValueTuple((int x, int y) posA, (int, int) diff, int offset)
    {
        (int, int) found;
        found = (posA.Item1 + diff.Item1 * offset, posA.Item2 + diff.Item2 * offset);
        return found;
    }

    // Helper function to check if a position is valid
    private static bool IsValidPosition((int, int) position, (int, int) size)
    {
        return position.Item1 >= 0 && position.Item2 >= 0 && position.Item1 < size.Item1 && position.Item2 < size.Item2;
    }
}
