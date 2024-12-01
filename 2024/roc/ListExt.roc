module [nums, getOrUnwrap]

nums : List Str -> List U32
nums = \list ->
    list
    |> List.map
        (\n ->
            when n |> Str.toU32 is
                Ok num -> num
                Err e -> crash "not a number, err: $(Inspect.toStr e)"
        )

getOrUnwrap : List a, U64 -> a
getOrUnwrap = \list, idx ->
    when List.get list idx is
        Ok elem -> elem
        Err _ -> crash "no element at this index"
