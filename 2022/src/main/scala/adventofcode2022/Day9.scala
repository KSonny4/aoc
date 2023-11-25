package adventofcode2022

object Day9 {

  def solve1(parsedInput: Seq[Seq[Int]]) = {
    parsedInput
  }

  lazy val input: Seq[Seq[Int]] = io.Source
    .fromInputStream(getClass.getResourceAsStream("day9.txt"))
    .getLines()
    .toSeq
    .map(_.toSeq.map(_.asDigit))

  def main(args: Array[String]): Unit = {
    println(solve1(input))
  }
}
