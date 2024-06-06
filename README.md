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

You can customize the output colors.

```sh
# -c <message color> <cat color>
kittysay -c 2 5 <message>
echo <message> | kittysay -c 2 5
```

### `--width`

You can use the `--width` flag to change the width of the speech bubble. Defaults to `45`, maxes out at a little less than the width of your terminal if you try to pass a very large number.

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

You can use the `--think` flag to enable "think" mode, where the speech bubbles are replaced with thought bubbles (similar to the `cowthink` program).

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

## License

[GPL-3.0](LICENSE)
