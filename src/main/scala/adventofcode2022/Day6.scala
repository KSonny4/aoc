package adventofcode2022

object Day6 {

  def solve1(parsedInput: String) =
    parsedInput
      .sliding(4)
      .toSeq
      .zipWithIndex
      .map(x => x._1.toSet -> x._2)
      .filter(x => x._1.toSeq.length == 4)
      .take(1)(0)
      ._2 + 4

  def solve2(parsedInput: String) =
    parsedInput
      .sliding(14)
      .toSeq
      .zipWithIndex
      .map(x => x._1.toSet -> x._2)
      .filter(x => x._1.toSeq.length == 14)
      .take(1)(0)
      ._2 + 14

  lazy val input: String = io.Source
    .fromInputStream(getClass.getResourceAsStream("day6.txt"))
    .getLines()
    .next()

  def main(args: Array[String]): Unit = {
    println(solve1(input))
    println(solve2(input))
  }
}
