# pkg
A cli frontend to emerge, written in Rust.

## Why?
Because. I can.

## Installation
Rust and Cargo are required. It is recommended to install them via [rustup](https://www.rust-lang.org/tools/install).

## Features
- gentoolkit: Enable this if you have gentoolkit installed and you can check the deps of packages.

### Manually with cargo
1: `cargo install --git https://github.com/Phate6660/pkg.git`

### From crates.io
`cargo install pkg-gentoo`

### From overlay
Note: This is currently outdated, will update soon.<br>
1. Install the overlay [p6nc-overlay](https://github.com/p6nc/overlay).
2. Ensure repos and overlays are synced: `sudo emerge --sync`.
3. `sudo emerge -atv pkg-gentoo`

Note: Even though the package is named `pkg-gentoo`, the binary is still `pkg`.

## Usage
`$ pkg -h`
```
pkg 0.0.9
Phate6660 <https://github.com/Phate6660>
A cli frontend for emerge, plus some extra features. Why? Because I can.

Note: Package operations require root.
So run with your preferred method of privilege elevation, otherwise emerge will ask if you want to pretend.

USAGE:
    pkg [FLAGS] [OPTIONS]

FLAGS:
    -c, --clean      Remove any un-needed packages.
    -h, --help       Prints help information
    -l, --list       List currently installed packages.
    -p, --portup     One-shot emerge portage. Used when you need to specifically update portage.
    -S, --sync       Update repos and overlays.
    -u, --update     Update any installed packages.
    -V, --version    Prints version information
    -w, --world      Prints the contents of your world file.

OPTIONS:
    -d, --deps <PKG>              View the dependencies of a package. Note: Requires gentoolkit to be installed.
    -f, --frem <PKGS>...          Force remove package(s).
    -i, --install <PKGS>...       Install package(s).
    -m, --metadata <PKG>          View the metadata of a package. Note: Requires gentoolkit to be installed.
    -r, --remove <PKGS>...        Remove package(s).
    -s, --search <SEARCHES>...    Search for package(s).
```

## Roadmap
There's really nothing concrete. I'm just adding and editing features as I see fit right now.<br>
If anyone actually wants to use this and would like to see features added, feel free to open an issue.<br>
Or if you want to contribute anything, feel free to open a PR.
