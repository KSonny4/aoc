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

  
    static void Solve(string filePath)
    {
        var content = File.ReadAllText(filePath);

        List<List<char>> data = content
            .Split(new[] { "\n" }, StringSplitOptions.RemoveEmptyEntries)
            .Select(line => line.ToList())
            .ToList();



        
    }

    


}
