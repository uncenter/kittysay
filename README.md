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

#### Try it out with nix

```sh
nix run github:uncenter/kittysay/v0.3.0 ":3"
# or for the latest version
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

## License

[MIT](LICENSE)
