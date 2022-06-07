# 0.3.0
## 2022-06-07

- Update to 2021 edition of Rust (no syntactical changes needed)
- Update all library dependencies
- The `crossbeam` upgrade requires changing `use` paths in `main.rs` and `render.rs` (from `crossbeam::...` to `crossbeam::channel::...`)
- The `crossterm` upgrade requires adding a `use` statement to bring `crossterm::style::Stylize` into scope in `render.rs`

# 0.2.0
## 2020-03-13

- Update to 2018 edition of Rust (from 2015 edition)
- Update all library dependencies
- Break audio handling into an independent [`rusty_audio`] library
- Add Windows compatibility by switching from `termion` to `crossterm` for the terminal library
- Use `crossbeam` channels instead of `mpsc` channels.
- Update the readme instructions, add gameplay screencast
- Lots and lots and lots of cleanup.

[`rusty_audio`]: https://github.com/cleancut/rusty_audio

# Version 0.1.0 - OpenWest
## 2017-06-25 to 2017-07-12

- Present at OpenWest conference: _Rusty Sword: Game of Infamy!  ***Live-code a game
  in Rust in 45 minutes.***_
- Compatible only with macOS and Linux
