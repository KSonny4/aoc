using System.Collections.Concurrent;

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
                .Select((c, x) => new { c, x, y })
                .Where(item => item.c != '.')
            )
            .GroupBy(item => item.c)
            .ToDictionary(
                group => group.Key,
                group => group.Select(item => (item.x, item.y)).ToList()
            );

        var resultTuple = (result, (width, height));

        var positions = resultTuple.Item1
            .SelectMany(item =>
                item.Value
                    .SelectMany((posA, i) =>
                        item.Value
                            .Skip(i + 1)
                            .Select(posB => new { posA, posB })
                    )
                    .Select(pair =>
                    {
                        var diff = (pair.posA.Item1 - pair.posB.Item1, pair.posA.Item2 - pair.posB.Item2);

                        var foundA = (pair.posA.Item1 + diff.Item1, pair.posA.Item2 + diff.Item2);
                        var foundB = (pair.posB.Item1 - diff.Item1, pair.posB.Item2 - diff.Item2);

                        // Return foundA and foundB as a List of Tuples
                        return new List<(int, int)> { foundA, foundB };
                    })
                    .SelectMany(pair => pair.Where(found =>
                        found.Item1 >= 0 && found.Item2 >= 0 && found.Item1 < resultTuple.Item2.Item1 && found.Item2 < resultTuple.Item2.Item2)
                    )
            )
            .ToList();

        var found = positions.Distinct().ToList();
        Console.WriteLine($"Found {found.Count} positions");
    }
}
