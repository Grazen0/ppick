# ppick · [![Tests](https://github.com/Grazen0/ppick/actions/workflows/tests.yml/badge.svg)](https://github.com/Grazen0/ppick/actions/workflows/tests.yml)

A simple, no-fuss CLI picker menu.

[ ![asciicast](https://asciinema.org/a/708394.svg) ](https://asciinema.org/a/708394)

It matches entries by *prefix* (hence the name, as in "prefix-pick") and exits the program as soon as there is only one selectable entry left.

## Usage

Entries (newline-separated) are piped in via stdin, and the selected entry is piped via stdout. For example:

```bash
result=$(printf "foo\nbar\nbaz" | ppick)
```

Use `ppick --help` to find more about the possible flags.

### Features

- [ ] Supports Unicode (probably already does, but needs testing).
- [ ] Allows showing the current query.

## Development

If you use [Nix](https://nixos.org/), you already know what to do (wink).

If not, just use Cargo. Everything should work out of the box.

## Credits

- [mini.starter](https://github.com/echasnovski/mini.starter) - main inspiration.
- [fzf](https://github.com/junegunn/fzf) - reference for program behavior.
- [yazi](https://github.com/sxyazi/yazi) - reference for man pages and shell completion generation.
