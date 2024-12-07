using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Diagnostics; // Add this namespace for Stopwatch

public class OperatorPermutations
{
    private static readonly string[] Operators = { "+", "*", "||" };

    // Functional approach to generating all permutations of given count
    public static IEnumerable<List<string>> GetOperatorPermutations(int count)
    {
        // Base case: If count is zero, return an empty list
        if (count == 0)
        {
            return new[] { new List<string>() };
        }

        // Recursive case: Generate permutations for count-1, then prepend each operator
        return GetOperatorPermutations(count - 1)
            .SelectMany(prevPerm => Operators, 
                (prevPerm, op) => prevPerm.Concat(new[] { op }).ToList());
    }
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

    static void Solve(string filePath)
    {
        // Start measuring time
        Stopwatch stopwatch = Stopwatch.StartNew();

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

        // Stop measuring time and display the elapsed time
        stopwatch.Stop();
        Console.WriteLine($"Execution Time: {stopwatch.ElapsedMilliseconds} ms");
    }

    static Dictionary<int, List<List<string>>> GenerateOperatorPermutations(List<int> numberLists) =>
        numberLists
            .ToDictionary(
                cnt => cnt, // Use the length of the list as the key
                cnt =>  OperatorPermutations.GetOperatorPermutations(cnt - 1).ToList() // Generate and collect permutations
            );


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
                        result = long.Parse(result.ToString() + numbers[i + 1].ToString());
                        break;                    
                    default:
                        throw new InvalidOperationException($"Unsupported operator: {signList[i]}");
                }
                if (result > target)
                {
                    return false;
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
