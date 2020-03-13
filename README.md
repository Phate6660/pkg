# pkg
A cli frontend to emerge, written in Rust.

## Why?
Because. I can.

## Usage
`$ pkg -h`
```
pkg 0.0.1
Phate6660 <https://github.com/Phate6660>
A cli frontend for emerge, plus some extra features. Why? Because I can.

USAGE:
    pkg [FLAGS] [OPTIONS]

FLAGS:
    -c, --clean      Remove any un-needed packages.
    -h, --help       Prints help information
    -l, --list       List currently installed packages.
    -p, --portup     One-shot emerge portage. Used when you need to specifically update porage.
    -S, --sync       Update repos and overlays.
    -V, --version    Prints version information
    -w, --world      Prints the contents of your world file.

OPTIONS:
    -f, --frem <PKGS>...          Force remove package(s).
    -i, --install <PKGS>...       Install package(s).
    -r, --remove <PKGS>...        Remove package(s).
    -s, --search <SEARCHES>...    Search for package(s).
```

## Roadmap
There's really nothing concrete. I'm just adding and editing features as I see fit right now.<br>
If anyone actually wants to use this and would like to see features added, feel free to open an issue.<br>
Or if you want to contribute anything, feel free to open a PR.
