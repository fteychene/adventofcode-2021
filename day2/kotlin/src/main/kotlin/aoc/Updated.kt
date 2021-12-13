package aoc

import arrow.core.*

object Updated {
    sealed class Command
    data class Forward(val value: Int): Command()
    data class Up(val value: Int): Command()
    data class Down(val value: Int): Command()

    data class Pos(val depth: Int, val horizontal: Int, val aim: Int)

    fun parseCommandValue(input: String, toCommand: (Int) -> Command): Either<Throwable, Command> =
        input.split(" ").lastOrNone().toEither { IllegalArgumentException("$input forward is not well formated") }
            .flatMap { value -> Either.catch { value.toInt() } }
            .map { toCommand(it) }

    fun parseCommand(input: String): Either<Throwable, Command> =
        when {
            input.startsWith("forward") -> parseCommandValue(input) { Forward(it) }
            input.startsWith("up") -> parseCommandValue(input) { Up(it) }
            input.startsWith("down") -> parseCommandValue(input) { Down(it) }
            else -> IllegalArgumentException("$input is not a proper command").left()
        }

    fun applyCommand(pos: Pos, command: Command): Pos =
        when(command) {
            is Down -> pos.copy(aim = pos.aim + command.value)
            is Forward -> pos.copy(horizontal = pos.horizontal + command.value, depth = pos.depth + (command.value * pos.aim))
            is Up -> pos.copy(aim = pos.aim - command.value)
        }

    fun run(inputs: String): Either<Throwable, Int> =
        inputs.lines()
            .traverseEither(this::parseCommand)
            .map { it.fold(Pos(0, 0, 0), this::applyCommand) }
            .map { it.depth * it.horizontal }
}