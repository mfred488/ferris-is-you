**Ferris Is You** is a clone of the awesome game [Baba Is You](https://hempuli.itch.io/baba), which is *a puzzle game about changing the rules*. Go buy it if you're into puzzle game, you won't regret it!

This open-source clone started as a hobby project, to get me into the [Rust](https://www.rust-lang.org/) programming language. So instead of using Baba, it uses [Ferris](https://rustacean.net/) (the unofficial mascot for Rust) as the main character!
Out of respect for the original game, I don't plan to release this game with the original levels of *Baba Is You*.

The project is at the very early stage of its development. It only runs in a terminal, and supports only a limited subset of the words of *Baba Is You*

## Supported words
* Object names (Ferris, Flag, Rocket, Wall, etc.)
* is (limited support: only for *noun* is *adjective*)
* you
* win
* stop
* push
* sink
* float

## Not (yet) supported words
* all
* empty
* hot/melt
* defeat
* shift
* more
* move
* open
* shut
* has
* not
* on
* and
* near
* lonely
* fall

## Backlog:

- [ ] Keep implementing additional words
- [ ] Implement waiting option
- [x] Implement level loader (i.e. build a level from an Unicode description of the level)
- [ ] Add tests
- [ ] Restructure the code (only one monolithic file as of now) (**ongoing**)
- [ ] Build a WebAssembly version, and make it run in the browser

## Things that will probably never be done

* Support of word "level"
* "Map" levels
* Individual characters ?
