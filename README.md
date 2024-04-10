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
# or
cargo install --git https://github.com/uncenter/kittysay.git
```

### Nix

#### Try it out

```sh
nix run github:uncenter/kittysay/v0.5.0 ":3"
# or for the latest commit
nix run github:uncenter/kittysay -- ":3"
```

<details>

<summary>

#### Installation with flakes

</summary>

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    kittysay.url = "github:uncenter/kittysay";
  };

  outputs = { self, nixpkgs, kittysay }: {
    nixosConfigurations.example = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [{
        environment.systemPackages = [
          inputs.kittysay.packages.${pkgs.system}.default
        ];
      }];
    };
  }
}
```

</details>

## Usage

```sh
kittysay <message>
# or using stdin
echo <message> | kittysay -
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
 kittysay "meow mrrrow mrrrp nyaaa nya nyaaa meow meowwww nyaaa meowwww" --width 1

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
