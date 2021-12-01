@file:JvmName("AOC2021")

package com.grappenmaker.aoc2021

import java.io.File
import java.nio.charset.Charset

fun main(args: Array<String>) {
    val index = args.getOrNull(0)?.toInt() ?: throw IllegalArgumentException("Illegal index")
    println("Running solution for day $index (https://adventofcode.com/2021/day/$index)")
    println()

    when (index) {
        1 -> solveDay1()
        else -> println("Couldn't find solution for day $index")
    }
}

// Util to get input
fun getInputFile(day: Int) = File("inputs", "day-${day.toString().padStart(2, '0')}.txt")
fun getStream(day: Int) = getInputFile(day).inputStream()
fun getInput(day: Int): String = String(getStream(day).readBytes(), Charset.defaultCharset())
fun getInputLines(day: Int): List<String> = getStream(day).bufferedReader().readLines()