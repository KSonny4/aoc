ThisBuild / version := "0.1.0-SNAPSHOT"

ThisBuild / scalaVersion := "2.13.10"
libraryDependencies += "org.scalaz" %% "scalaz-core" % "7.3.6"

lazy val root = (project in file("."))
  .settings(
    name := "aoc"
  )
