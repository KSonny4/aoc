package adventofcode2022

object Day3 {

  def charToInt(char: Char) =
    char match {
      case x if x.isUpper => x.toInt - 38
      case x if x.isLower => x.toInt - 96
    }

  def solve1(parsedInput: Seq[String]) =
    parsedInput
      .map(x =>
        (
          x.slice(0, x.length / 2).toSet,
          x.slice(x.length / 2, x.length).toSet
        )
      )
      .flatMap(x => x._1.intersect(x._2).toSeq)
      .map(charToInt)
      .sum

  def solve2(parsedInput: Seq[String]) =
    parsedInput
      .grouped(3)
      .flatMap { case Seq(a, b, c) =>
        a.toSet.intersect(b.toSet).intersect(c.toSet)
      }
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
