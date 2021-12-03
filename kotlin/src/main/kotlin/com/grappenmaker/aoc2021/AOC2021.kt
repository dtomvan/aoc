@file:JvmName("AOC2021")

package com.grappenmaker.aoc2021

import java.io.File
import java.nio.charset.Charset
import kotlin.system.measureTimeMillis

fun main(args: Array<String>) {
    val breaker = "=".repeat(20)

    val index = args.getOrNull(0)?.toInt() ?: throw IllegalArgumentException("Illegal index")
    println("Running solution for day $index (https://adventofcode.com/2021/day/$index)")
    println()
    println(breaker)

    measureTimeMillis {
        when (index) {
            1 -> solveDay1()
            2 -> solveDay2()
            3 -> solveDay3()
            else -> println("Couldn't find solution for day $index")
        }
    }.also {
        println(breaker)
        println()
        println("Took ${it}ms to run solution.")
    }
}

// Util to get input
fun getInputFile(day: Int) = File("inputs", "day-${day.toString().padStart(2, '0')}.txt")
fun getStream(day: Int) = getInputFile(day).inputStream()
fun getInput(day: Int): String = String(getStream(day).readBytes(), Charset.defaultCharset())
fun getInputLines(day: Int): List<String> = getStream(day).bufferedReader().readLines()