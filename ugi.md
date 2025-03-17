## General
- The engine must always be able to process input, even while searching.
- The engine must never take an action without being commanded to do so.
- The engine will always be given a position before being asked to search.
- Option "OwnBook" must be false by default
- Only three commands may be sent while an engine is searching: `isready`, `stop`, and `quit`
- All messages must end with a newline.

---

## ugi
- Tell the engine to use the UGI protocol
- The engine must immediately reply to this command with the following:
  - `id name <name>`
  - `id author <author>`
- The engine must next list all options it has available:
  - `option name <name> type <type> default <default>`
- The engine must finish with the following:
  - `ugiok`

---

## setoption name <name> value <value>
Set the option with the name given to the value given.
Examples:
- `setoption name Hash value 1000`
- `setoption name Threads value 4`

---

## isready
Ask the engine whether it's ready to receive further commands. This is used after the engine has been asked to do something potentially time intensive that may not have finished yet.
- Will always be sent once after `ugiok` is received and any options have been set with `setoption`.
- Will always be sent once after `uginewgame` is sent.
- Will always be sent once after a new position has been sent.

---

## uginewgame
`uginewgame`
- Will always be sent once when a new game has been started.
- The engine must clear all data related to any previous game.
- An `isready` command must always be sent immediately afterwards.

---

## position <startpos | fen \<fen>> [moves \<moves>]
Set the current position to either the starting position or FEN string provided, then apply any moves given to the position. The `moves` parameter will only be included with a nonempty sequence of moves.

Examples:
- `position startpos`
- `position startpos moves e2e4 c7c5`
- `position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`
- `position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4 c7c5`

---

## go
Start searching the current position. Only one of five different ways to search will be specified. All arguments must be included for the relevant search type. `go` will never be called on a position without legal moves. Upon finishing, search must print `bestmove <x>` where `x` is a string representation of the best move found.

Time:
- `p1time <x>` - The amount of time player 1 has left in milliseconds > 0.
- `p2time <x>` - The amount of time player 2 has left in milliseconds > 0.
- `p1inc <x>` - The amount of increment player 1 is given in milliseconds >= 0.
- `p2inc <x>` - The amount of increment player 2 is given in milliseconds >= 0.

Movetime:
- `movetime <x>` - The amount of time to search for in milliseconds > 0.

Depth:
- `depth <x>` - The maximum depth to search to >= 0.

Nodes:
- `nodes <x>` - The maximum number of nodes to search > 0.

Infinite:
- `infinite` - Search until `stop` is received, or until naturally forced to stop by algorithmic or implementation limitations.

Examples:
- `go p1time 1000 p2time 1000 p1inc 10 p2inc 10`
- `go movetime 500`
- `go depth 3`
- `go nodes 5000`
- `go infinite`

---

## info
`info` strings relay updates on the state of an ongoing search. They should be sent regularly. One final `info` string should be sent once the search has finished but before `bestmove` containing `nodes`, `time`, and `nps`.
- `depth <x>` - The current depth of the search.
- `seldepth <x>` - The deepest the search has selectively looked.
- `time <x>` - The amount of time spent searching so far in milliseconds.
- `nodes <x>` - The number of nodes visited.
- `hashfull <x>` - Approximately how full is the hash table in parts per thousand.
- `nps <x>` - The number of nodes being searched per second.
- `multipv <x>` - The rank of the current PV being shown. Rank starts at 1 for the strongest pv and increments from there.
- `score`
  - `cp <x>` - The score in centipoints from the engine's point of view.
  - `mate <x>` - Mate in <x> plies, not moves. Negative values if the engine is being mated.
  - `lowerbound` - The score is a lower bound.
  - `upperbound` - The score is an upper bound.
- `pv <moves>` - The best sequence of moves found.

---

## query <x>
Due to being game agnostic, UGI relies on players to provide information about the game state. When a query is sent, the engine must reply with `response`.
- `p1turn` - Is it currently player one's turn?
- `gameover` - Is the game over for the current position?
- `result` - What's the result for the current position?

---

## response <x>
When a `query` is received, the engine must reply with the relevant information.
- `p1turn` - Return `true` or `false` depending on whether it's player one's turn.
- `gameover` - Return `true` or `false` depending on whether the current position is game over.
- `result` - Return `p1win`, `p2win`, `draw`, or `none` depending on the current position's game over result.

Example:
```
query p1turn
response false
query gameover
response true
query result
response p1win
```

---

## stop
Stop any ongoing search as soon as possible. The search must still print output as expected before stopping.

---

## quit
The engine process must end as soon as possible. Any ongoing searches must finish normally.
