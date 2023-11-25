package adventofcode2022

object Day3 {

  def charToInt(char: Char) =
    char match {
      case x if x.isUpper => x.toInt - 38
      case x if x.isLower => x.toInt - 96
    }

  def solve1(parsedInput: Seq[String]) =
    parsedInput
      .map(x => x.splitAt(x.length / 2))
      .flatMap(x => x._1.toSet.intersect(x._2.toSet).toSeq)
      .map(charToInt)
      .sum

  def solve2(parsedInput: Seq[String]) =
    parsedInput
      .grouped(3)
      .flatMap(_.map(_.toSet).reduce(_.intersect(_)))
      .map(charToInt)
      .sum

  lazy val input: Seq[String] = io.Source
    .fromInputStream(getClass.getResourceAsStream("day3.txt"))
    .getLines()
    .toSeq

  def main(args: Array[String]): Unit = {
    println(solve1(input))
    println(solve2(input))
  }
}
