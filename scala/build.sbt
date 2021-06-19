ThisBuild / scalaVersion     := "3.0.0"
ThisBuild / organization     := "info.rgomes"
ThisBuild / organizationName := "frgomes"
ThisBuild / name             := "ring-buffer"

// -Yexplicit-nulls

lazy val dependencies : Seq[Setting[_]] =
  Seq(
    libraryDependencies += "net.jcip" % "jcip-annotations" % "1.0",
  )

lazy val uTestFramework : Seq[Setting[_]] =
  Seq(
    libraryDependencies += "com.lihaoyi" %% "utest" % "0.7.10" % "test",
    testFrameworks += new TestFramework("utest.runner.Framework")
  )

lazy val root = (project in file("."))
  .settings(name := (ThisBuild / name).value)
  .settings(dependencies)
  .settings(uTestFramework)
