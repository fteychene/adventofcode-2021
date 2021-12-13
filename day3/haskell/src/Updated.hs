module Updated
    ( run
    ) where

import Data.Foldable (foldl')
import Data.Char (digitToInt)
import GHC.Conc (par)

parseInput :: String -> [String]
parseInput x = lines x

count :: (Int, Int) -> Char -> (Int, Int)
count (x, y) z = case z of
    '1' -> (x, y+1)
    _ -> (x+1, y)

-- >>> oxygenFilter '1' (1, 1)
-- True
oxygenFilter:: Char -> (Int, Int) -> Bool
oxygenFilter char (zeroCount, oneCount) = case char of
        '1' -> oneCount >= zeroCount
        _ -> zeroCount > oneCount

co2Filter:: Char -> (Int, Int) -> Bool
co2Filter char (zeroCount, oneCount) = case char of
        '0' -> zeroCount <= oneCount
        _ -> oneCount < zeroCount

-- >>> filterWithCounter oxygenFilter (4, 3) 0 ["101000111100", "000011111101", "011100000100", "100100010000", "100100010000"]
filterWithCounter:: (Char -> (Int, Int) -> Bool) -> (Int, Int) -> Int -> [String] -> [String]
filterWithCounter p counter index = filter (\a -> p (a !! index) counter)

-- >>> columnCount 1 ["11110","10110","10111","10101","11100","10000","11001"]
-- (4,3)
columnCount :: Int -> [String] -> (Int, Int)
columnCount index = foldl (\acc x -> count acc (x !! index)) (0,0)

-- >>> filterLines ["11110", "10110", "10111", "10101", "11100", "10000", "11001"] oxygenFilter 0
-- "10111"
filterLines :: [String] -> (Char -> (Int, Int) -> Bool) -> Int -> String
filterLines lines predicate i = case lines of
    [] -> ""
    [x] -> x
    x -> let counter = columnCount i x
         in filterLines (filterWithCounter predicate counter i x) predicate (i+1)

toDec :: String -> Int
toDec = foldl' (\acc x -> acc * 2 + digitToInt x) 0

run :: String -> Either String Int
run input = let oxygen = toDec $ filterLines (parseInput input) oxygenFilter 0
                co2 = toDec $ filterLines (parseInput input) co2Filter 0
                in Right (oxygen * co2)
