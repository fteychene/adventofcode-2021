object Initial {

  def run(inputs: String): Either[Throwable, Int] = {
    Right(inputs
      .linesIterator
      .map(x => x.toInt)
      .sliding(2)
      .count(x => x.head < x.tail.head))
  }

}
