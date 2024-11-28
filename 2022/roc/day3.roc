app [main] { pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.16.0/O00IPk-Krg_diNS2dVWlI0ZQP794Vctxzv0ha96mK0E.tar.br" }

import pf.Stdout

import "./../input/day3.txt" as input : Str

main =
    Stdout.line! (part1 input |> Num.toStr)
    Stdout.line! (part2 input |> Result.withDefault 0 |> Num.toStr)

asciiToPriority = \n ->
    if n >= 97 then n - 96 else n - 64 + 26

compartments = \items -> List.splitAt items ((List.len items) // 2) |> \c -> { l: c.before, r: c.others }

nonUniqueItems = \{ l, r } -> Set.intersection (Set.fromList l) (Set.fromList r) |> Set.toList

part1 = \in ->
    in
    |> Str.trim
    |> Str.splitOn "\n"
    |> List.map (\rucksack -> rucksack |> Str.toUtf8 |> List.map Num.toU32 |> List.map asciiToPriority)
    |> List.map
        (\rucksack -> rucksack
            |> compartments
            |> nonUniqueItems
            |> List.sum)
    |> List.sum

findBadges = \rucksacks ->
    { before, others } = List.splitAt rucksacks 1
    others |> List.walk (List.first? before) (\state, elem -> Set.intersection state elem) |> Ok

part2 = \in ->
    in
        |> Str.trim
        |> Str.splitOn "\n"
        |> List.map (\rucksack -> rucksack |> Str.toUtf8 |> List.map Num.toU32 |> List.map asciiToPriority |> Set.fromList)
        |> List.chunksOf 3
        |> List.mapTry? findBadges
        |> List.map (\badges -> badges |> Set.toList |> List.sum)
        |> List.sum
        |> Ok

testInput =
    """
    vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw
    """

expect part1 testInput == 157

expect part2 testInput |> Result.withDefault 0 == 70
