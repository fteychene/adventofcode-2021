module Lib
    ( execute
    ) where

import qualified Initial
import qualified Updated
import Text.Printf (printf)

execute :: IO ()
execute = do
    inputs <- readFile "inputs"
    _ <- putStrLn $ case Initial.run inputs of
        Right value -> "Initial : " ++ show value
        Left e      -> "Initial error : " ++ show e
    _ <- putStrLn $ case Updated.run inputs of
        Right value -> "Updated : " ++ show value
        Left e      -> "Updated error : " ++ show e
    return ()