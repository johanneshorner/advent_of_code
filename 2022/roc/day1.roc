app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.16.0/O00IPk-Krg_diNS2dVWlI0ZQP794Vctxzv0ha96mK0E.tar.br" }

import pf.Stdout

import "./../input/day1.txt" as input : Str

result1 = part1 input
result2 = part2 input

main =
    Stdout.line!
        "$(Num.toStr result1)\n$(Num.toStr result2)"

part1 = \in ->
    in
    |> Str.trim
    |> Str.splitOn "\n\n"
    |> List.map
        (\elve -> Str.splitOn elve "\n"
            |> List.map (\food -> Str.toU32 food |> Result.withDefault 0)
            |> List.sum)
    |> List.max
    |> Result.withDefault 0

part2 = \in ->
    in
    |> Str.trim
    |> Str.splitOn "\n\n"
    |> List.map
        (\elve -> Str.splitOn elve "\n"
            |> List.map (\food -> Str.toU32 food |> Result.withDefault 0)
            |> List.sum)
    |> List.sortDesc
    |> (\cals -> List.dropLast cals (List.len cals - 3))
    |> List.sum

testInput =
    """
    1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    """

expect
    part1 testInput == 24000

expect
    part2 testInput == 45000
