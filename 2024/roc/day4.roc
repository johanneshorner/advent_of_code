app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.16.0/O00IPk-Krg_diNS2dVWlI0ZQP794Vctxzv0ha96mK0E.tar.br" }

import pf.Stdout

import "./../input/day1.txt" as input : Str

main =
    Stdout.line! (part1 input |> Inspect.toStr)

part1 = \in ->
    rangePairs =
        in
            |> Str.trim
            |> Str.splitOn "\n"

testInput =
    """
    """

expect part1 testInput == 0
