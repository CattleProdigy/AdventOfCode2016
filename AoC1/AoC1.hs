import Data.Char
import System.IO

getDigitMatchesHelper :: [Int] -> [Int] -> Int -> [Int] -> [Int]
getDigitMatchesHelper (i:input) (c:circ) lastVal matchesSoFar
    | i == lastVal = getDigitMatchesHelper input circ i (i:matchesSoFar)
    | i /= lastVal = getDigitMatchesHelper input circ i (matchesSoFar)
    | otherwise = matchesSoFar

-- When input list is exhausted, check the wraparound
getDigitMatchesHelper [] (c:circ) lastVal matchesSoFar
    | c == lastVal = (c:matchesSoFar)
    | otherwise = matchesSoFar

getDigitMatches :: [Int] -> [Int]
getDigitMatches x = getDigitMatchesHelper x circular (-1) []  
    where circular = x ++ [head x] -- same list, but with the first element
                                   -- for checking wrap-around

-- For the types
readInt :: Char -> Int
readInt x = read [x]

main = (print . sum . getDigitMatches . preprocess) =<< (getLine)
    where preprocess = map readInt . filter isDigit
