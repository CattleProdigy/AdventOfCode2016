import Data.Char
import System.IO

shift :: [a] -> Int -> [a]
shift list n = drop n list ++ take n list

getDigitMatchesHelper :: [Int] -> [Int] -> [Int] -> [Int]
getDigitMatchesHelper (i:input) (c:circ) matchesSoFar
    | i == c = getDigitMatchesHelper input circ (i:matchesSoFar)
    | i /= c = getDigitMatchesHelper input circ (matchesSoFar)
    | otherwise = matchesSoFar

getDigitMatchesHelper [] [] matchesSoFar = matchesSoFar

getDigitMatches :: [Int] -> [Int]
getDigitMatches x = getDigitMatchesHelper x halfShift []
    where halfShift = shift x (quot (length x) 2)

-- For the types
readInt :: Char -> Int
readInt x = read [x]

main = (print . sum . getDigitMatches . preprocess) =<< (getLine)
    where preprocess = map readInt . filter isDigit
