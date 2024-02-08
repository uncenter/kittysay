# kittysay

The cutest successor of [`cowsay`](https://en.wikipedia.org/wiki/Cowsay).

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
nix run github:uncenter/kittysay/v0.3.0 ":3"
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

```
kittysay ":3"

echo ":3" | kittysay -
```

You can use the `--width` flag to change the width of the message box (`kittysay ":3" --width 100`), or the `--think` flag to enable "think" mode (in which the speech bubbles are replaced with thought bubbles, similar to the `cowthink` program).

## License

[GPL-3.0](LICENSE)
