module [toNums]

toNums : List Str -> List U32
toNums = \list ->
    list
    |> List.map
        (\n ->
            when n |> Str.toU32 is
                Ok num -> num
                Err e -> crash "not a number, err: $(Inspect.toStr e)"
        )
