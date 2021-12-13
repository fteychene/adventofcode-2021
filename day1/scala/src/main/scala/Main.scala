import java.io.FileNotFoundException
import scala.io.Source
import scala.util.Try

@main def main: Unit =
  val inputs = readInputs("inputs")
  inputs.flatMap(inputs => Initial.run(inputs))
    .foreach(x => println(s"Initial : $x"))
  inputs.flatMap(inputs => Updated.run(inputs))
    .foreach(x => println(s"Updated : $x"))

def readInputs(resourcePath: String): Either[Throwable, String] =
  Try(Source.fromResource(resourcePath).mkString)
    .recover(e => throw new IllegalStateException(s"Can't read inputs $resourcePath", e))
    .fold(Left(_), Right(_))

