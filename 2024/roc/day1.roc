app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.16.0/O00IPk-Krg_diNS2dVWlI0ZQP794Vctxzv0ha96mK0E.tar.br" }

import pf.Stdout
import ListExt

import "./../input/day1.txt" as input : Str

main =
    Stdout.line! (part1 input |> Inspect.toStr)
    Stdout.line! (part2 input |> Inspect.toStr)

part1 = \in ->
    in
    |> Str.trim
    |> Str.splitOn "\n"
    |> List.walk
        ([], [])
        (\state, line ->
            nums = Str.splitOn line "   " |> ListExt.nums
            (
                List.append state.0 (ListExt.getOrUnwrap nums 0),
                List.append state.1 (ListExt.getOrUnwrap nums 1),
            )
        )
    |> \(l, r) ->
        List.map2 (List.sortAsc l) (List.sortAsc r) (\left, right -> Num.absDiff left right)
    |> List.sum

part2 = \in ->
    (l, r) =
        in
        |> Str.trim
        |> Str.splitOn "\n"
        |> List.walk
            ([], [])
            (\state, line ->
                nums = Str.splitOn line "   " |> ListExt.nums
                (
                    List.append state.0 (ListExt.getOrUnwrap nums 0),
                    List.append state.1 (ListExt.getOrUnwrap nums 1),
                )
            )

    l
    |> List.map (\n -> (n, List.countIf r (\listN -> n == listN)))
    |> List.map (\(n, count) -> (Num.toU64 n) * count)
    |> List.sum

testInput =
    """
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    """

expect part1 testInput == 11
expect part2 testInput == 31
