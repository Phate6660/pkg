# pkg
A cli frontend to emerge, written in Rust.

## Why?
Because. I can.

## Usage
`$ pkg -h`
```
pkg 0.0.1
Phate6660 <https://github.com/Phate6660>
A cli frontend for emerge. Why? Because I can.

Note: Cleaning, installing, and removing packages are automatically confirmed.
Use at your own risk.

USAGE:
    pkg [FLAGS] [OPTIONS]

FLAGS:
    -c, --clean      Remove any un-needed packages.
    -h, --help       Prints help information
    -l, --list       List currently installed packages.
    -S, --sync       Update repos and overlays.
    -V, --version    Prints version information
    -w, --world      Prints the contents of your world file.

OPTIONS:
    -i, --install <PKGS>...       Install package(s).
    -r, --remove <PKGS>...        Remove package(s).
    -s, --search <SEARCHES>...    Search for package(s).
```

## Roadmap
There's really nothing concrete. I'm just adding and editing features as I see fit right now.<br>
If anyone actually wants to use this and would like to see features added, feel free to open an issue.<br>
Or if you want to contribute anything, feel free to open a PR.
