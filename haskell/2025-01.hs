import Foreign.Marshal.Utils

parseLine :: String -> Int
parseLine (d : n) = read n * if d == 'L' then -1 else 1

doDial :: Int -> Int -> Int
doDial dial x = (dial + x) `mod` 100

doZeVakkingScheiße :: (Int, Int, Int) -> Int -> (Int, Int, Int)
doZeVakkingScheiße (p1, p2, dial) x =
    ( p1 + iszero
    , p2 + spillage + wentzero
    , newdial
    )
  where
    newdial = doDial dial x
    iszero = fromBool $ newdial == 0
    spillage = (abs $ dial + x) `div` 100
    wentzero = fromBool $ dial + x <= 0 && dial /= 0

main = do
    contents <- getContents
    let input = map parseLine $ lines contents
    let (turns, passes, _) = foldl doZeVakkingScheiße (0, 0, 50) input

    print turns
    print passes
