# 0.2.0
## unreleased

- Update to 2018 edition of Rust (from 2015 edition)
- Break audio handling into an independent [`rusty_audio`] library
- Add Windows compatibility by switching from `termion` to `crossterm` for the terminal library
- Use `crossbeam` channels instead of `mpsc` channels.
- Update the readme instructions, add gameplay screencast

[`rusty_audio`]: https://github.com/cleancut/rusty_audio

# Version 0.1.0 - OpenWest
## 2017-06-25 to 2017-07-12

- Present at OpenWest conference: _Rusty Sword: Game of Infamy!  ***Live-code a game
  in Rust in 45 minutes.***_
- Compatible only with macOS and Linux