## General
- The engine MUST always be able to process input, even while searching.
- The engine MUST NOT take an action without being commanded to do so.
- The engine will always be given a position before being asked to search.
- Option "OwnBook" MUST be false by default
- Only three commands MAY be sent while an engine is searching: `isready`, `stop`, and `quit`
- All messages MUST end with a newline.

---

## ugi
- Tell the engine to use the UGI protocol
- The engine MUST immediately reply to this command with the following:
  - `id name <name>`
  - `id author <author>`
- The engine MUST next list all options it has available, one per line, using one of the following formats depending on the option's type:
  - `option name <name> type check default <true | false>`
  - `option name <name> type spin default <default> min <min> max <max>`
  - `option name <name> type combo default <default> var <value1> var <value2> ...`
  - `option name <name> type button`
  - `option name <name> type string default <default>`
- The engine MUST finish with the following:
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
- The engine MUST clear all data related to any previous game.
- An `isready` command MUST always be sent immediately afterwards.

---

## position <startpos | fen \<fen>> [moves \<moves>]
Set the current position to either the starting position or FEN string provided, then apply any moves given to the position. The `moves` parameter will only be included with a nonempty sequence of moves.

Examples:
- `position startpos`
- `position startpos moves e2e4 c7c5`
- `position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`
- `position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4 c7c5`

---

## moves <move> [move ...]
Apply one or more moves to the current position. This is the same mechanism used by the `moves` part of the `position` command, and MAY also be sent on its own to advance the current position further without resending it in full.

Examples:
- `moves e2e4`
- `moves e2e4 c7c5`

---

## go
Start searching the current position. Only one of five different ways to search will be specified. All arguments MUST be included for the relevant search type. `go` will never be called on a position without legal moves. Upon finishing, search MUST print `bestmove <x>` where `x` is a string representation of the best move found.

Time:
- `p1time <x>` - The amount of time player 1 has left in milliseconds > 0.
- `p2time <x>` - The amount of time player 2 has left in milliseconds > 0.
- `p1inc <x>` - The amount of increment player 1 is given in milliseconds >= 0.
- `p2inc <x>` - The amount of increment player 2 is given in milliseconds >= 0.

Movetime:
- `movetime <x>` - The amount of time to search for in milliseconds > 0.

Movestogo (OPTIONAL, combined with the time parameters above):
- `movestogo <x>` - The number of moves remaining until the next time control >= 1.

Depth:
- `depth <x>` - The maximum depth to search to >= 0.

Nodes:
- `nodes <x>` - The maximum number of nodes to search > 0.

Infinite:
- `infinite` - Search until `stop` is received, or until naturally forced to stop by algorithmic or implementation limitations.

Examples:
- `go p1time 1000 p2time 1000 p1inc 10 p2inc 10`
- `go p1time 60000 p2time 60000 movestogo 40`
- `go movetime 500`
- `go depth 3`
- `go nodes 5000`
- `go infinite`

---

## perft [depth] <x>
Count the total number of leaf nodes reachable after exactly `x` plies from the current position, used to verify move generation is correct. The `depth` keyword is OPTIONAL.
- `x` - The depth to search to >= 1.

Examples:
- `perft 5`
- `perft depth 5`

---

## split [depth] <x>
As `perft`, but additionally reports the leaf node count contributed by each individual legal move available in the current position.
- `x` - The depth to search to >= 1.

Due to being game agnostic, UGI does not guarantee that a position has a finite number of legal moves. `split` is only well-defined when the legal moves at the current position can be enumerated; if they cannot, the engine SHOULD report the problem via an `info string` message (see Errors) instead of attempting to list them.

Examples:
- `split 5`
- `split depth 5`

---

## options
Reprint the engine's available options, in the same format used during the `ugi` handshake.

---

## movelist
Print the list of legal moves available in the current position. Intended for debugging move generation, not for use by a match runner.

Due to being game agnostic, UGI does not guarantee that a position has a finite number of legal moves. `movelist` is only well-defined when the legal moves at the current position can be enumerated; if they cannot, the engine SHOULD report the problem via an `info string` message (see Errors) instead of attempting to list them.

---

## info
`info` strings relay updates on the state of an ongoing search. They SHOULD be sent regularly. One final `info` string SHOULD be sent once the search has finished but before `bestmove` containing `nodes`, `time`, and `nps`.
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
- `string <text>` - A free-form message intended for human consumption, e.g. debug output. `<text>` runs to the end of the line. This can be sent standalone as `info string <text>` at any time, not just as part of a search.

---

## query <x>
Due to being game agnostic, UGI relies on players to provide information about the game state. When a query is sent, the engine MUST reply with `response`.
- `p1turn` - Is it currently player one's turn?
- `gameover` - Is the game over for the current position?
- `result` - What's the result for the current position?

---

## response <x>
When a `query` is received, the engine MUST reply with the relevant information.
- `p1turn` - Return `true` or `false` depending on whether it's player one's turn.
- `gameover` - Return `true` or `false` depending on whether the current position is game over.
- `result` - Return `p1win`, `p2win`, `draw`, or `none` depending on the current position's game over result.

---

## stop
Stop any ongoing search as soon as possible. The search MUST still print output as expected before stopping.

---

## quit
The engine process MUST end as soon as possible. Any ongoing searches MUST finish normally.

---

## Errors
If a command is malformed or otherwise cannot be processed, the engine SHOULD report the problem with an `info string` message rather than crashing, hanging, or exiting, e.g. `info string position error <message>`. Any other unrecognised input MUST be silently ignored. The engine MUST NOT exit or leave its state corrupted as a result of a bad command.
