# ppick

A simple, no-fuss TUI picker menu.

It matches entries by *prefix* (hence the name, as in "prefix-pick") and exits the program as soon as there is only one selectable entry left.

## Usage

Menu entries are piped in via stdin. For example:

```bash
printf "foo\nbar\nbaz" | ppick
```

## Development

If you use [Nix](https://nixos.org/), you already know what to do (wink).

If not, just use Cargo. Everything should work out of the box.

## Credit

- [mini.starter](https://github.com/echasnovski/mini.starter) - main inspiration.
- [fzf](https://github.com/junegunn/fzf) - reference for program behavior.
- [yazi](https://github.com/sxyazi/yazi) - reference for man pages and shell completion generation.
