module Main where

import System.Environment (getArgs)
import Control.Monad (fmap, forM)
import Control.Concurrent.MVar
import Control.Concurrent (forkIO, threadDelay)

main = do
  [ philosophers ] <- fmap ( map ( read :: String -> Int ) ) getArgs
  forks <- forM [1..philosophers] newMVar
  let forkPairs = zip forks $ tail $ cycle forks
      sort = cycle [ id, \(a,b) -> (b,a) ]
      sortedForkPairs = [ f pair | (f, pair) <- zip sort forkPairs ]
  mapM_ startPhilosopher sortedForkPairs
  readLn :: IO String
  
startPhilosopher (first, second) =
  forkIO $ philospherLoop first second
  
philospherLoop first second = do
  [ a, b ] <- mapM takeMVar [ first, second ]
  threadDelay 100
  putMVar second b
  putMVar first a
  philospherLoop first second