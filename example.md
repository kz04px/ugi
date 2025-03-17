| Command from the user | Response from the engine |
| - | - |
| `ugi`<br>Tell the engine to use the UGI protocol. ||
|| `id name Engine`<br>`id author Author`<br>The engine tells us its name is `Engine` and its author is `Author`. |
|| `option name Debug type check default false`<br>The engine tells us it has a `check` option called `Debug` with a default value of `false` |
|| `ugiok`<br>The engine has finished telling us about itself. |
| `setoption name Debug value true`<br>We tell the engine to set its `Debug` option to `true` ||
| `isready`<br>We ask the engine if it's ready. This is the point where any large memory allocations or other time intensive tasks must run. ||
|| `readyok`<br>All startup tasks have finished and the engine is ready to play. |
| `uginewgame`<br>Inform the engine that this is a new game. ||
| `position startpos moves e2e4 c7c5`<br>Set the current position to `startpos` and then apply the moves `e2e4` and `c7c5`. ||
| `isready`<br>We ask the engine if it's ready. ||
|| `readyok`<br>The engine has updated the position and is ready to search. |
| `go depth 3`<br>Tell the engine to search to a depth of 3. ||
|| `info depth 1 seldepth 1 score cp 0 nodes 10 pv move1`<br>`info depth 2 seldepth 3 score cp 10 nodes 150 pv move1 move2`<br>`info depth 3 seldepth 7 score cp 20 nodes 2130 pv move1 move3 move4`<br>The engine has optionally given us information about its search for the first 3 depths searched to. |
|| `info nodes 2130 time 123`<br>Once the search is completed, the engine sends a final `info` string. |
|| `bestmove move1`<br>The engine reports that the best move it has found is `move1` |
| `query result`<br>Ask the engine what the game result is. ||
|| `response none`<br>There is no game result as the game is not currently finished. |
| `quit`<br>Tell the engine to quit. ||
