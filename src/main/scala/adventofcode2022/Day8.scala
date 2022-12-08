package adventofcode2022
import scala.collection.immutable.Seq

object Day8 {

  def check(
      forrest: Seq[Seq[Int]],
      indices: (Int, Int)
  ) = {
    val imax = forrest.length - 1
    val jmax = forrest.head.length - 1

    if (indices._1 == forrest.length - 1 || indices._1 == 0) 1
    else if (indices._2 == forrest.head.length - 1 || indices._2 == 0) 1
    else {
      val i = indices._1
      val j = indices._2
      val positions = Seq(
        (i to i) -> (j + 1 to jmax),
        (i + 1 to imax) -> (j to j),
        (i to i) -> (0 until j),
        (0 until i) -> (j to j)
      )
      if (
        positions
          .map(position =>
            position._1.flatMap(i => position._2.map(j => i -> j))
          )
          .map(position =>
            position.map(y => forrest(y._1)(y._2) < forrest(i)(j))
          )
          .map(x => x.forall(_ == true))
          .exists(_ == true)
      ) 1
      else 0
    }
  }

  def lookSurrounding(forrest: Seq[Seq[Int]], indices: (Int, Int)): Int =
    if (check(forrest, indices) == 1) 1
    else 0

  def solve(
      forrest: Seq[Seq[Int]],
      indices: Seq[(Int, Int)]
  ): Int = indices.map(x => lookSurrounding(forrest, x)).sum

  def solve1(parsedInput: Seq[Seq[Int]]) = {
    val rangei = 0 to parsedInput.length - 1
    val rangej = 0 to parsedInput.head.length - 1

    val x = rangei.flatMap(i => rangej.map(j => i -> j))
    solve(parsedInput, x)
  }

  lazy val input: Seq[Seq[Int]] = io.Source
    .fromInputStream(getClass.getResourceAsStream("day8.txt"))
    .getLines()
    .toSeq
    .map(_.toSeq.map(_.asDigit))

  def main(args: Array[String]): Unit = {
    println(solve1(input))
  }
}
