module Initial
    ( run
    ) where
import Data.Foldable (foldl')
import Data.Char (digitToInt)

--- >>> parseInput "Coucou\nCa va?\nMoi je le sens aps cette histoire de Haskell"
-- ["Coucou","Ca va?","Moi je le sens aps cette histoire de Haskell"]
parseInput :: String -> [String]
parseInput x = lines x

-- 101000111100
-- 000011111101
-- 011100000100
-- 100100010000

augment :: (Int, Int) -> Char -> (Int, Int)
augment (x, y) z = case z of
    '1' -> (x, y+1)
    _ -> (x+1, y)

--- >>> augmentByLine [(0,0), (0,0), (0,0), (0,0)] ['1', '0', '1', '1']
-- [(0,1),(1,0),(0,1),(0,1)]
augmentByLine ::[(Int, Int)] -> [Char] -> [(Int, Int)]
augmentByLine = zipWith augment
-- map (\x y -> augment x y) $ zip x y TODO why u no work


-- >>> countBits ["101000111100", "000011111101", "011100000100"]
-- [(2,1),(2,1),(1,2),(2,1),(2,1),(2,1),(1,2),(1,2),(1,2),(0,3),(3,0),(2,1)]
countBits:: [String] -> [(Int, Int)]
countBits = foldl augmentByLine $ take 12 $ repeat (0, 0)

toDec :: String -> Int
toDec = foldl' (\acc x -> acc * 2 + digitToInt x) 0

gamma :: [(Int, Int)] -> Int
gamma x = toDec $ map (\ (zeroCount, oneCount) -> if oneCount > zeroCount then '1' else '0') x

epsilon :: [(Int, Int)] -> Int
epsilon x = toDec $ map (\ (zeroCount, oneCount) -> if oneCount < zeroCount then '1' else '0') x


run :: String -> Either String Int
run x = let bits = countBits $ parseInput x
            gammaV = gamma bits
            epsilonV = epsilon bits
            in Right (gammaV * epsilonV)
