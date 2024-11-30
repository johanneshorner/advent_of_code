app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.16.0/O00IPk-Krg_diNS2dVWlI0ZQP794Vctxzv0ha96mK0E.tar.br" }

import pf.Stdout

import "./../input/day4.txt" as input : Str

main =
    Stdout.line! (part1 input |> Inspect.toStr)
    Stdout.line! (part2 input |> Inspect.toStr)

areFullyContained = \r1, r2 ->
    (r1.start >= r2.start && r1.end <= r2.end)
    ||
    (r2.start >= r1.start && r2.end <= r1.end)

areOverlapping = \r1, r2 ->
    (
        (r1.start >= r2.start && r1.start <= r2.end)
        ||
        (r1.end <= r2.end && r1.end >= r2.start)
    )
    ||
    (
        (r2.start >= r1.start && r2.start <= r1.end)
        ||
        (r2.end <= r1.end && r2.end >= r1.start)
    )

toRangePairs = \line ->
    toU32orZero = \n -> n |> Str.toU32 |> Result.withDefault 0

    rangeFromStr = \str ->
        str
        |> Str.splitFirst "-"
        |> Result.map (\{ before, after } -> { start: before |> toU32orZero, end: after |> toU32orZero })
        |> Result.withDefault { start: 0, end: 0 }

    line
        |> Str.splitFirst? ","
        |> \{ before, after } -> (before |> rangeFromStr, after |> rangeFromStr)
        |> Ok

part1 = \in ->
    rangePairs =
        in
            |> Str.trim
            |> Str.splitOn "\n"
            |> List.mapTry? toRangePairs

    rangePairs
    |> List.countIf (\(first, second) -> areFullyContained first second)
    |> Ok

part2 = \in ->
    rangePairs =
        in
            |> Str.trim
            |> Str.splitOn "\n"
            |> List.mapTry? toRangePairs

    rangePairs
    |> List.countIf (\(first, second) -> areOverlapping first second)
    |> Ok

testInput =
    """
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    """

expect part1 testInput == Ok 2

expect part2 testInput == Ok 4
