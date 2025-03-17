# The Universal Game Interface - WIP
The Universal Game Interface (UGI) protocol is a game agnostic way to enable games to be played between engines without having to implement the game rules in the match runner.

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

## Examples
Examples can be found [here](./example.md)

---

## Software using UGI
- [CuteGames](https://github.com/kz04px/cutegames) - A match runner.
- [Faeries](https://github.com/kz04px/faeries) - An engine to play various games.
