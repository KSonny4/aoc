package adventofcode2022

object Day4 {

  def solve1(parsedInput: Seq[(Range, Range)]) =
    parsedInput
      .map { x: (Range, Range) =>
        x._2.toSet.intersect(x._1.toSet) == x._1.toSet || x._1.toSet.intersect(
          x._2.toSet
        ) == x._2.toSet
      }
      .count(x => x)

  def solve2(parsedInput: Seq[(Range, Range)]) =
    parsedInput
      .map { x: (Range, Range) =>
        x._2.toSet.intersect(x._1.toSet).nonEmpty || x._1.toSet
          .intersect(
            x._2.toSet
          )
          .nonEmpty
      }
      .count(x => x)

  def createRange(stringRange: String): Range =
    stringRange.split("-").map(_.toInt).toSeq match {
      case Seq(x, y) => x to y
      case _         => throw new Exception("bad input")
    }

  lazy val input: Seq[(Range, Range)] = io.Source
    .fromInputStream(getClass.getResourceAsStream("day4.txt"))
    .getLines()
    .map(x =>
      x.split(",").toSeq match {
        case Seq(x, y) => createRange(x) -> createRange(y)
        case _         => throw new Exception("bad input")
      }
    )
    .toSeq

  def main(args: Array[String]): Unit = {
    println(solve1(input))
    println(solve2(input))

  }
}
