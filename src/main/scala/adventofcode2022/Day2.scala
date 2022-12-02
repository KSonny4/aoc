package adventofcode2022

object Day2 {

  sealed trait Game
  sealed trait Hand extends Game

  object Rock extends Hand

  object Paper extends Hand

  object Scissors extends Hand

  sealed trait PossibleEndings extends Game

  object Lose extends PossibleEndings

  object Draw extends PossibleEndings

  object Win extends PossibleEndings

  val substitutions: Map[String, Hand] = Map(
    "A" -> Rock,
    "B" -> Paper,
    "C" -> Scissors,
    "X" -> Rock,
    "Y" -> Paper,
    "Z" -> Scissors
  )
  val substitutionsPart2: Map[String, Game] = Map(
    "A" -> Rock,
    "B" -> Paper,
    "C" -> Scissors,
    "X" -> Lose,
    "Y" -> Draw,
    "Z" -> Win
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

  def parseInput2(input: String) =
    input
      .split("\n")
      .toSeq
      .map(_.split(" ").toSeq match {
        case Seq(x, y) =>
          substitutionsPart2.getOrElse(
            x,
            throw new Exception("bad input")
          ) -> substitutionsPart2.getOrElse(y, throw new Exception("bad input"))
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

  def solve2(parsedInput: Seq[(Game, Game)]) =
    parsedInput.map {
      case (Rock, Draw)     => 3 + 1
      case (Paper, Draw)    => 3 + 2
      case (Scissors, Draw) => 3 + 3

      case (Rock, Win)     => 6 + 2
      case (Paper, Win)    => 6 + 3
      case (Scissors, Win) => 6 + 1

      case (Rock, Lose)     => 0 + 3
      case (Paper, Lose)    => 0 + 1
      case (Scissors, Lose) => 0 + 2
    }.sum

  lazy val input: String = io.Source
    .fromInputStream(getClass.getResourceAsStream("day2.txt"))
    .mkString
    .trim

  def main(args: Array[String]): Unit = {
    println(solve1(parseInput(input)))
    println(solve2(parseInput2(input)))

  }
}
