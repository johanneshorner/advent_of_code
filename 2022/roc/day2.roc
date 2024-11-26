app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.16.0/O00IPk-Krg_diNS2dVWlI0ZQP794Vctxzv0ha96mK0E.tar.br" }

import pf.Stdout

import "./../input/day2.txt" as input : Str

main =
    Stdout.line! (part1 input |> Result.withDefault 0 |> Num.toStr)

rps = \{ l, r } ->
    when [l, r] is
        ["A", "X"] -> Draw
        ["A", "Y"] -> Win
        ["A", "Z"] -> Loss
        ["B", "X"] -> Loss
        ["B", "Y"] -> Draw
        ["B", "Z"] -> Win
        ["C", "X"] -> Win
        ["C", "Y"] -> Loss
        ["C", "Z"] -> Draw
        _ -> crash "impossible"

sToN = \s ->
    when s is
        "X" -> 1
        "Y" -> 2
        "Z" -> 3
        _ -> crash "impossible"

part1 = \in ->
    in
        |> Str.trim
        |> Str.splitOn "\n"
        |> List.mapTry? (\round -> Str.splitFirst? round " ")
        |> List.map
            (\{ before: l, after: r } ->
                (
                    when rps { l, r } is
                        Win -> 6
                        Draw -> 3
                        Loss -> 0
                )
                + (sToN r)
            )
        |> List.sum
        |> Ok

testInput =
    """
    A Y
    B X
    C Z
    """

expect
    part1 testInput |> Result.withDefault 0 == 15

# expect
#     part2 testInput == 12
