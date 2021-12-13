package aoc

import arrow.core.Either
import arrow.core.flatMap
import arrow.core.zip

object Flag

fun main() {
    val inputs = Either.catch {
        Flag::class.java.classLoader.getResource("inputs")?.readText(Charsets.UTF_8)!!
    }.mapLeft { IllegalStateException("Can't read inputs", it) }

    val initial = inputs
        .flatMap { Initial.run(it) }
        .tap { println("Initial : $it") }

    val updated = inputs
        .flatMap { Updated.run(it) }
        .tap { println("Updated : $it") }

    initial.zip(updated)
        .tapLeft { it.printStackTrace() }
}