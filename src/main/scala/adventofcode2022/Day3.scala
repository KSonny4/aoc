package adventofcode2022

object Day3 {

  def parseInput(input: String) =
    input
      .split("\n")
      .toSeq

  def charToIntSum(chars: Seq[Char]) =
    chars.map {
      case x if x.isUpper => x.toInt - 38
      case x if x.isLower => x.toInt - 96
    }.sum

  def solve1(parsedInput: Seq[String]) =
    charToIntSum(
      parsedInput
        .map(x =>
          (
            x.slice(0, x.length / 2).toSet,
            x.slice(x.length / 2, x.length).toSet
          )
        )
        .flatMap(x => x._1.intersect(x._2).toSeq)
    )

  def solve2(parsedInput: Seq[String]) =
    charToIntSum(
      parsedInput
        .grouped(3)
        .flatMap { case Seq(a, b, c) =>
          a.toSet.intersect(b.toSet).intersect(c.toSet)
        }
        .toSeq
    )
  lazy val input: String = io.Source
    .fromInputStream(getClass.getResourceAsStream("day3.txt"))
    .mkString
    .trim

  def main(args: Array[String]): Unit = {
    println(solve1(parseInput(input)))
    println(solve2(parseInput(input)))
  }
}
