# The Universal Game Interface - WIP
The Universal Game Interface (UGI) protocol is a game agnostic way to enable games to be played between engines without having to implement the game rules in the match runner.

---

## Terminology
The key words "must", "must not", "required", "shall", "shall not", "should", "should not", "recommended", "may", and "optional" in the specification are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## Game compatability
Games must be:
- Two player
- Have nonsimultaneous turns

---

## The specification
Details on the protocol can be found [here](./ugi.md)

---

## FEN and move strings
Due to being game agnostic, UGI has no notion of what FEN or move strings should look like for a given game. As such, it only requires that any interacting software be consistent.

---

## Match runners
A match runner is not required to know the rules of the game being played. It may simply relay `position`/`moves` between the two engines, using the `query`/`response` commands to ask an engine whose turn it is, whether the game is over, and what the result was, rather than working any of that out itself.

---

## Examples
Examples can be found [here](./example.md)

---

## Reference implementation
A Rust reference implementation of the protocol lives in this repository — see [`src/`](./src) and [`tests/`](./tests), or `cargo build`/`cargo test` from the repo root.

A minimal tic-tac-toe engine built on it is at [`examples/tic_tac_toe.rs`](./examples/tic_tac_toe.rs), runnable with `cargo run --example tic_tac_toe`.

---

## Custom extensions
The Rust reference implementation's `UGI` trait has an `unrecognised_command` hook that gets called for any top-level command it doesn't recognise. This lets an engine built on top of it add its own custom, non-standard commands without forking the implementation. It is **not** part of the UGI specification — the spec itself only requires that unrecognised input be silently ignored, and any extension commands built on this hook are only guaranteed to be understood by that specific engine.

---

## Games
Popular games that fit UGI's compatibility requirements, split by whether a match runner needs to know the game's rules to run a match.

### Match runner doesn't need to know the rules
- chess
- checkers
- othello
- connect four
- go
- shogi
- xiangqi
- gomoku
- nine men's morris
- tic-tac-toe

### Match runner needs to know the rules
- backgammon - the runner must generate dice rolls itself and embed them into the position, since UGI has no built-in notion of randomness
- Scrabble (2 player) - the runner must draw tiles itself and embed each player's rack into the position, since UGI has no built-in notion of randomness or hidden information
- poker (heads-up) - the runner must shuffle and deal cards itself and embed each player's hole cards into the position, since UGI has no built-in notion of randomness or hidden information
- Uno (2 player) - the runner must shuffle and deal cards itself and embed each player's hand into the position, since UGI has no built-in notion of randomness or hidden information
- Risk (2 player) - the runner must control the neutral third army and its dice rolls itself, since UGI has no notion of a non-participating player

---

## Incompatible games
Popular games that don't fit UGI's compatibility requirements:
- Settlers of Catan - more than two players, no official 2-player variant
- Diplomacy - more than two players, simultaneous turns
- rock-paper-scissors - simultaneous turns

---

## Software using UGI
- [CuteGames](https://github.com/kz04px/cutegames) - A match runner.
- [Faeries](https://github.com/kz04px/faeries) - An engine to play various games.
