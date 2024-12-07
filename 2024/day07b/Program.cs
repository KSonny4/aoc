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
            "../../../input_test.txt"
        );
        Solve(inputFilePath);
    }

    static void Solve(string filePath)
    {
        var content = File.ReadAllText(filePath);

        var data = content
            .Split(new[] { "\n" }, StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.Split(new[] { ":" }, StringSplitOptions.RemoveEmptyEntries))
            .Select(strings =>
                Tuple.Create(
                    long.Parse(strings.First()), // Use long here
                    strings.Last().Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToList() // Use long here
                )
            );

        var uniqueNumberListLengths = data
            .Select(x => x.Item2.Count) // Count the number of elements in each list
            .Distinct() // Get the unique counts
            .ToList(); // Convert the result to a list

        var permutations_signs = GenerateOperatorPermutations(uniqueNumberListLengths);

        var matchingItems = data
            .AsParallel()
            .Where(tuple => ComputeAndCheck(permutations_signs[tuple.Item2.Count], tuple.Item2, tuple.Item1)) // Use long here
            .Select(tuple => tuple.Item1) // Select the first item as long
            .Sum(); // Sum of matching items

        Console.WriteLine(matchingItems);
    }

    static Dictionary<int, List<List<string>>> GenerateOperatorPermutations(List<int> numberLists) =>
        numberLists
            .ToDictionary(
                cnt => cnt, // Use the length of the list as the key
                cnt => GetOperatorPermutations(cnt - 1).ToList() // Generate and collect permutations
            );

    static IEnumerable<List<string>> GetOperatorPermutations(int count) =>
        Enumerable.Range(0, 1 << count) // Generate numbers from 0 to 2^count - 1
            .Select(i => Convert.ToString(i, 2).PadLeft(count, '0')) // Convert each number to binary
            .Select(binary => binary.Select(bit => bit == '0' ? "+" : (bit == '1' ? "*" : "||")).ToList()); // Map bits to operators, adding `||` for third option

    static bool ComputeAndCheck(List<List<string>> signs, List<long> numbers, long target) // Use long here
    {
        foreach (var signList in signs) // Loop through each list of signs
        {
            long result = numbers[0]; // Start with the first number as long

            for (int i = 0; i < signList.Count; i++)
            {
                switch (signList[i])
                {
                    case "+":
                        result += numbers[i + 1];
                        break;
                    case "*":
                        result *= numbers[i + 1];
                        break;
                    case "||":
                        // Concatenate the numbers as strings and then parse them as long
                        result = long.Parse(result.ToString() + numbers[i + 1].ToString());
                        break;
                    default:
                        throw new InvalidOperationException($"Unsupported operator: {signList[i]}");
                }
            }

            // If the result matches the target, return true immediately
            if (result == target)
            {
                return true;
            }
        }

        // If no match was found, return false
        return false;
    }
}
