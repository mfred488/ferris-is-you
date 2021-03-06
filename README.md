**Ferris Is You** is a clone of the awesome game [Baba Is You](https://hempuli.itch.io/baba), which is *a puzzle game about changing the rules*. Go buy it if you're into puzzle game, you won't regret it!

This open-source clone started as a hobby project, to get me into the [Rust](https://www.rust-lang.org/) programming language. So instead of using Baba, it uses [Ferris](https://rustacean.net/) (the unofficial mascot for Rust) as the main character!
Out of respect for the original game, I don't plan to release this game with the original levels of *Baba Is You*.

The project is at the very early stage of its development. It only runs in a terminal, and supports only a limited subset of the words of *Baba Is You*.

## Supported words
* Object names (Ferris, Flag, Rocket, Wall, etc.)
* is (limited support: we don't detect conflicting rules for now)
* you
* win
* stop
* push
* sink
* float
* move
* defeat
* hot/melt
* shift
* open / shut
* and (limited support: only A and B is C, or A is B and C)
* has
* weak
* pull
* tele
* swap
* fall
* on
* near
* facing
* lonely

## Not (yet) supported words
* all
* empty
* more
* not
* make (Garden 10)
* word
* up/right (Forest E, 6, 8)

## Backlog:

* Core:
- [ ] Keep implementing additional words
- [ ] Understand and implement contradictory rules settlement
- [ ] Find some complex examples (cf TODOs in test files)
- [x] Implement waiting option
- [x] Implement level loader (i.e. build a level from an Unicode description of the level)
- [x] Add tests
- [x] Restructure the code (only one monolithic file as of now)

* Browser version:
- [x] Basic version
- [ ] Graphical polish (font, text on 2 lines, emphasize active rules, etc.)
- [ ] Level loading
- [ ] Saving progress in local storage
- [ ] Adding sounds and animations ?
- [ ] Level editor

* Unicode version:
- [ ] Display orientation of elements (especially needed for move/shift)

## Things that will probably never be done

* Support of words "level" and "cursor"
* "Map" levels
* Individual characters ?
