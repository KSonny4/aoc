package adventofcode2022

object Day1 {

  def parseInput(input: String) =
    input
      .split("\n\n")
      .map(_.split("\n").map(_.toInt))

  def solve1(parsedInput: Array[Array[Int]]): Int =
    parsedInput.map(_.sum).max

  def solve2(parsedInput: Array[Array[Int]]): Int =
    parsedInput
      .map(_.sum)
      .sorted
      .reverse
      .take(3)
      .sum

  lazy val input: String = io.Source
    .fromInputStream(getClass.getResourceAsStream("day1.txt"))
    .mkString
    .trim

  def main(args: Array[String]): Unit = {
    println(solve1(parseInput(input)))
    println(solve2(parseInput(input)))
  }
}
