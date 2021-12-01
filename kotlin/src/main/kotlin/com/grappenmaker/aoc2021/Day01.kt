package com.grappenmaker.aoc2021

fun solveDay1() {
    // Part one
    val input = getInputLines(1).map { it.toInt() }
    val countIncrements = { arr: List<Int> ->
        arr.filterIndexed { i, n -> if (i == 0) false else n > arr[i - 1] }.size
    }

    val increments = countIncrements(input)
    println("Part one: $increments")

    // Part two
    val sumIncrements =
        countIncrements(input.mapIndexed { i, _ -> if (i + 2 >= input.size) 0 else input.slice(i..i + 2).sum() })
    println("Part two: $sumIncrements")
}