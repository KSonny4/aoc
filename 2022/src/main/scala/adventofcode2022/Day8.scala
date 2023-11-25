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
          .exists(x => x.forall(_ == true))
      ) 1
      else 0
    }
  }

  def countTrues(x: IndexedSeq[Boolean]) = {
    val t = x.takeWhile(_ == true).length
    if (x.length == t) t
    else t + 1
  }

  def check2(
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
      // tohle check
      val positions = Seq(
        (i to i) -> (j + 1 to jmax),
        (i + 1 to imax) -> (j to j).reverse,
        (i to i) -> (j - 1 to 0 by -1),
        (i - 1 to 0 by -1) -> (j to j)
      )

      val a =
        positions
          .map(position =>
            position._1.flatMap(i => position._2.map(j => i -> j))
          )
          .map(position =>
            // s"pos: {($i)($j): ${forrest(i)(j)}}" -> position.map(y =>
            position.map(y => forrest(i)(j) > forrest(y._1)(y._2))
          )
          .map(x => countTrues(x))
          .product
      //println(a)
      //1
      a
    }
  }

  def solve(
      forrest: Seq[Seq[Int]],
      indices: Seq[(Int, Int)]
  ): Int = indices.map(x => check(forrest, x)).sum

  def solve1(parsedInput: Seq[Seq[Int]]) = {
    val rangei = 0 to parsedInput.length - 1
    val rangej = 0 to parsedInput.head.length - 1

    val x = rangei.flatMap(i => rangej.map(j => i -> j))
    solve(parsedInput, x)
  }

  def solve22(
      forrest: Seq[Seq[Int]],
      indices: Seq[(Int, Int)]
  ): Int = {
    val x = indices.map(x => check2(forrest, x))
    //println(x)
    x.max
  }

  def solve2(parsedInput: Seq[Seq[Int]]) = {
    val rangei = 0 to parsedInput.length - 1
    val rangej = 0 to parsedInput.head.length - 1

    val x = rangei.flatMap(i => rangej.map(j => i -> j))
    solve22(parsedInput, x)
  }

  lazy val input: Seq[Seq[Int]] = io.Source
    .fromInputStream(getClass.getResourceAsStream("day8.txt"))
    .getLines()
    .toSeq
    .map(_.toSeq.map(_.asDigit))

  def main(args: Array[String]): Unit = {
    println(solve1(input))
    println(solve2(input))
  }
}
