# Tempy

Convert temperature values from fahrenheit to celsius and vice versa.

## Why?
This was built to get some practice with concepts from chapters 1-3 of the rust book.

## Prerequisites
[Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Compatibility
It's rust so it should just work on any of the big three? There's some docs on
Windows exceptions I ain't quite read yet...

## Uninstall
**Cargo**
```sh
rm -r /path/to/tempy
```

**Nix**
```sh
nix-store --gc
```

## Install
**Cargo**
Download the repository:

```sh
git clone git@github.com/fuguesoft/tempy
```

**Nix/NixOS (Flake)**

1. Enable nix flakes in `configuration.nix` 
```nix
nix.settings.experimental-features = [ "nix-command" "flakes" ];
```

or either `/etc/nix/nix.conf` or `$XDG_CONFIG_HOME/nix/nix.conf`
```conf
experimental-features = nix-command flakes
```

2. Add flake input to `/etc/flake.nix`
```nix

inputs = {
  # ...
  tempy = {
    url = "github:fuguesoft/tempy"
      };
  # ...
};
```

3. Expose flake input in `configuration.nix` or `home.nix`
configuration.nix
```nix
environment.systemPackages = with pkgs; [
  inputs.tempy.packages."${pkgs.stdenv.hostPlatform.system}".default
]
```

home.nix
```nix
home.packages = with pkgs; [
  inputs.tempy.packages."${pkgs.stdenv.hostPlatform.system}".default
]
```

## Run without installing
**Cargo**
Run the program from within the project root:
```sh
cd /path/to/tempy
cargo run
```

**Nix**
1. Enable Nix Flakes ([See Above](##Install))
2. Temporarily build and run without installing
```sh
nix run github:fuguesoft/tempy
```
