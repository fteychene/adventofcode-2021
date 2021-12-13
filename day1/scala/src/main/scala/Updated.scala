object Updated {

  def run(inputs: String): Either[Throwable, Int] =
    Right(inputs
      .linesIterator
      .map(x => x.toInt)
      .sliding(3)
      .map(x => x.sum)
      .sliding(2)
      .count(x => x.head < x.tail.head))
}
