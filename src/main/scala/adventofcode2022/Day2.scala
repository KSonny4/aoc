package adventofcode2022

object Day2 {

  sealed trait Hand

  object Rock extends Hand
  object Paper extends Hand
  object Scissors extends Hand

  val substitutions: Map[String, Hand] = Map(
    "A" -> Rock,
    "B" -> Paper,
    "C" -> Scissors,
    "X" -> Rock,
    "Y" -> Paper,
    "Z" -> Scissors
  )
  def parseInput(input: String) =
    input
      .split("\n")
      .toSeq
      .map(_.split(" ").toSeq match {
        case Seq(x, y) =>
          substitutions.getOrElse(
            x,
            throw new Exception("bad input")
          ) -> substitutions.getOrElse(y, throw new Exception("bad input"))
        case _ => throw new Exception("bad input")
      })

  def solve1(parsedInput: Seq[(Hand, Hand)]) =
    parsedInput.map {

      case (Rock, Paper)    => 6 + 2
      case (Rock, Scissors) => 3

      case (Paper, Scissors) => 6 + 3
      case (Paper, Rock)     => 1

      case (Scissors, Rock)  => 6 + 1
      case (Scissors, Paper) => 2

      case (Rock, Rock)         => 3 + 1
      case (Paper, Paper)       => 3 + 2
      case (Scissors, Scissors) => 3 + 3

    }.sum

  lazy val input: String = io.Source
    .fromInputStream(getClass.getResourceAsStream("day2.txt"))
    .mkString
    .trim

  def main(args: Array[String]): Unit = {
    println(solve1(parseInput(input)))

  }
}
