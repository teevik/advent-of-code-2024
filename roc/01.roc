app [main] {
    pf: platform "https://github.com/roc-lang/basic-cli/releases/download/0.17.0/lZFLstMUCUvd5bjnnpYromZJXkQUrdhbva4xdBInicE.tar.br",
    aoc: "https://github.com/lukewilliamboswell/aoc-template/releases/download/0.2.0/tlS1ZkwSKSB87_3poSOXcwHyySe0WxWOWQbPmp7rxBw.tar.br",
}

import pf.Stdin
import pf.Stdout
import pf.Utc
import aoc.AoC {
    stdin: Stdin.readToEnd,
    stdout: Stdout.write,
    time: \{} -> Utc.now {} |> Task.map Utc.toMillisSinceEpoch,
}

main =
    AoC.solve {
        year: 2024,
        day: 1,
        title: "Historian Hysteria",
        part1,
        part2,
    }

# exampleInput =
#    """
#    3   4
#    4   3
#    2   5
#    1   3
#    3   9
#    3   3
#    """

parseLine = \line ->
    parts = line |> Str.splitOn "   " |> List.map Str.toU32

    when parts is
        [Ok left, Ok right] -> { left, right }
        _ -> crash "invalid line"

parseInput = \input ->
    lines =
        input
        |> Str.trim
        |> Str.splitOn "\n"

    entries = lines |> List.map parseLine

    entries

part1 : Str -> Result Str _
part1 = \input ->
    entries = parseInput input

    lefts = entries |> List.map .left |> List.sortDesc
    rights = entries |> List.map .right |> List.sortDesc

    differences = List.map2 lefts rights Num.absDiff
    sum = List.sum differences

    Ok "The total distance is $(Num.toStr sum)"

part2 : Str -> Result Str _
part2 = \input ->
    # entries = parseInput input
    # pog : Dict U32 { left : U32, right : U32 }
    # pog = List.walk entries (Dict.empty {}) \state, elem ->
    #    updatedLeft = Dict.update state elem.left \possibleValue ->
    #        when possibleValue is
    #            Ok value -> Ok { left: value.left + 1, right: value.right }
    #            Err Missing -> Ok { left: 1u32, right: 0u32 }
    #    updatedRight = Dict.update updatedLeft elem.right \possibleValue ->
    #        when possibleValue is
    #            Ok value -> Ok { left: value.left, right: value.right + 1 }
    #            Err Missing -> Ok { left: 0u32, right: 1u32 }
    #    updatedRight

    # sum = Dict.walk pog 0u32 (\acc, key, value -> acc + (key * value.left * value.right))

    # Ok "The similarity score is $(Num.toStr sum)"

    entries = parseInput input

    occurences = List.walk entries (Dict.empty {}) \state, entry ->
        Dict.update state entry.right \possibleValue ->
            when possibleValue is
                Ok value -> Ok (value + 1u32)
                Err Missing -> Ok 1u32

    similarityScore =
        entries
        |> List.map .left
        |> List.map \element ->
            occurences
            |> Dict.get element
            |> Result.withDefault 0
            |> Num.mul element
        |> List.sum

    Ok "The similarity score is $(Num.toStr similarityScore)"
