# kittysay

[`cowsay`](https://en.wikipedia.org/wiki/Cowsay), but with a cute kitty :3

```

  ----
< meow >
  ----
  \
    \
      ／l、
    （ﾟ､ ｡ ７
      l  ~ヽ
      じしf_,)ノ

```

## Installation

### Cargo

```sh
cargo install kittysay
```

### Nix

[Available through Nixpkgs](https://nixpkgs.dev/kittysay).

```
nix run nixpkgs#kittysay
```

## Usage

```sh
kittysay <message>
# or through stdin
echo <message> | kittysay
```

You can customize the output colors for the message and cat each, using ANSI 256-bit colors:

```sh
kittysay --colors 2 5 <message>
echo <message> | kittysay --colors 2 5
```

Or use kittysay with your favorite command-line applications:

```sh
fortune | kittysay
```

### `--width`

You can use the `--width` flag (`-w` for short) to change the width of the speech bubble. Defaults to `45`, maxes out at a little less than the width of your terminal if you try to pass a very large number.

```
$ kittysay "meow mrrrow mrrrp nyaaa nya nyaaa meow meowwww nyaaa meowwww"

  --------------------------------------
/ meow mrrrow mrrrp nyaaa nya nyaaa meow \
\ meowwww nyaaa meowwww                  /
  --------------------------------------
  \
    \
      ／l、
    （ﾟ､ ｡ ７
      l  ~ヽ
      じしf_,)ノ
```

```
$ kittysay "meow mrrrow mrrrp nyaaa nya nyaaa meow meowwww nyaaa meowwww" --width 1000

  ------------------------------------------------------------
< meow mrrrow mrrrp nyaaa nya nyaaa meow meowwww nyaaa meowwww >
  ------------------------------------------------------------
  \
    \
      ／l、
    （ﾟ､ ｡ ７
      l  ~ヽ
      じしf_,)ノ

```

```
$ kittysay "meow mrrrow mrrrp nyaaa nya nyaaa meow meowwww nyaaa meowwww" --width 1

  -------
/ meow    \
| mrrrow  |
| mrrrp   |
| nyaaa   |
| nya     |
| nyaaa   |
| meow    |
| meowwww |
| nyaaa   |
\ meowwww /
  -------
  \
    \
      ／l、
    （ﾟ､ ｡ ７
      l  ~ヽ
      じしf_,)ノ

```

### `--think`

You can use the `--think` flag (`-t` for short) to enable "think" mode, where the speech bubbles are replaced with thought bubbles (similar to the `cowthink` program).

```
$ kittysay --think ":3"

  ⏜⏜
( :3 )
  ⏝⏝
  ○
    ○
      ／l、
    （ﾟ､ ｡ ７
      l  ~ヽ
      じしf_,)ノ

```

### `--colors`

You can customize the output colors for the message and cat each, using ANSI 256-bit colors, with the `--colors` flag (`-c` for short). For example, for a blue message and a pink cat, use `--colors 4 5`.

### `--tab-size`

To accurately draw the message box around the text, the width of tab characters must be considered. By default, widths are replaced with 4 spaces. To configure this, use the `--tab-size` flag. For example, to interpret tabs as two characters wide, `--tabs-size 2`.

## License

[GPL-3.0](LICENSE)
