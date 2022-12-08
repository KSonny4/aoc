package adventofcode2022

import scala.util.matching.UnanchoredRegex

case class File(name: String, size: Int)

case class Folder(
    name: String,
    files: Map[String, File] = Map.empty,
    folders: Map[String, Folder] = Map.empty
) {
  def setFolders(folders: Map[String, Folder]): Folder =
    copy(folders = folders)

  def setFiles(files: Map[String, File]): Folder =
    copy(files = files)
}

object Day7 {
  val cdBackPattern = "\\$ cd (..)".r
  val cdPattern = "\\$ cd (.*)".r
  val lsPattern = "(\\$ ls)".r
  val dirPattern = "dir (\\w*)".r
  val filePattern = "(\\d*) (.*)".r

  def solve(parsedInput: Seq[String], currentPosition: Folder): Any = {
    val row =
      if (parsedInput.length > 0) parsedInput.take(1)(0)
      else return currentPosition

    val a = row match {
      case cdBackPattern(dir) => dir
      case cdPattern(dir)     => dir
      case lsPattern(dir)     => solve(parsedInput.drop(1), currentPosition)
      case dirPattern(dir) =>
        solve(
          parsedInput.drop(1),
          currentPosition.copy(folders =
            currentPosition.folders + (dir -> Folder(name = dir))
          )
        )
      case filePattern(size, file) => file -> size
      case dunno                   => s"shit: $dunno"
    }
    println(a)
    solve(parsedInput.drop(1), currentPosition)
  }

  def solve1(parsedInput: Seq[String]) =
    solve(parsedInput.drop(1), Folder("/"))

  lazy val input: Seq[String] = io.Source
    .fromInputStream(getClass.getResourceAsStream("day7.txt"))
    .getLines()
    .toSeq

  def main(args: Array[String]): Unit = {
    println(solve1(input))
  }
}
