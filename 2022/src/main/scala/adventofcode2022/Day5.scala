package adventofcode2022

import scala.collection.immutable.ListMap

case class Move(moveNElements: Int, from: Int, to: Int)
object Day5 {

  def runSolve1(
      bins: Seq[Seq[Char]],
      commands: Seq[Move]
  ): Seq[Seq[Char]] = {
    if (commands.isEmpty) bins
    else {
      val Move(moveNElements, from, to) = commands.take(1)(0)
      val fromCargo = bins(from).slice(moveNElements, bins(from).length)
      val toCargo = bins(from).slice(0, moveNElements).reverse ++ bins(to)
      val modifiedBins = bins
        .updated(to, toCargo)
        .updated(from, fromCargo)
      runSolve1(modifiedBins, commands.drop(1))
    }
  }

  def runSolve2(
      bins: Seq[Seq[Char]],
      commands: Seq[Move]
  ): Seq[Seq[Char]] = {
    if (commands.isEmpty) bins
    else {
      val Move(moveNElements, from, to) = commands.take(1)(0)
      val fromCargo = bins(from).slice(moveNElements, bins(from).length)
      val toCargo = bins(from).slice(0, moveNElements) ++ bins(to)
      val modifiedBins = bins
        .updated(to, toCargo)
        .updated(from, fromCargo)
      runSolve2(modifiedBins, commands.drop(1))
    }
  }

  def solve1(
      parsedInput: (Seq[Seq[Char]], Seq[Move])
  ) = {
    runSolve1(parsedInput._1, parsedInput._2).flatMap(x => x.take(1))
  }

  def solve2(
      parsedInput: (Seq[Seq[Char]], Seq[Move])
  ) = {
    runSolve2(parsedInput._1, parsedInput._2).flatMap(x => x.take(1))
  }

  lazy val input: (Seq[Seq[Char]], Seq[Move]) = {
    val parsedInput = io.Source
      .fromInputStream(getClass.getResourceAsStream("day5.txt"))
      .getLines()
      .map(
        _.replaceAll("\\[", "")
          .replaceAll("]", "")
          .replaceAll("    ", "*")
          .replaceAll(" ", "")
      )
      .toSeq

    val spaceLocation = parsedInput.indexWhere(x => x == "")

    val (crates, movements) = parsedInput.splitAt(spaceLocation)
    val cratesBins =
      crates
        .dropRight(1)
        .map(_.toSeq)
        .map(_.zipWithIndex)
        .flatMap(_.filter { case (a: Char, _) => a != '*' })
        .groupBy(_._2)

    val sortedCratesBins = ListMap(cratesBins.toSeq.sortBy(_._1): _*).map {
      case (_, y) => y.map(_._1)
    }.toSeq

    val pattern = "move(\\d*)from(\\d*)to(\\d*)".r
    val movementCommands = movements
      .drop(1)
      .map {
        case pattern(count, from, to) =>
          Move(count.toInt, from.toInt - 1, to.toInt - 1)
        case x => throw new Exception(s"Could not parse $x")
      }

    println(cratesBins)
    (sortedCratesBins, movementCommands)
  }

  def main(args: Array[String]): Unit = {
    println(solve1(input))
    println(solve2(input))
  }
}
