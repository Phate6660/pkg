use clap::{App, Arg};
mod commands;
use commands::*;

fn main() {
    let matches = App::new("pkg")
		.version("0.0.9")
		.author("Phate6660 <https://github.com/Phate6660>")
		.about("A cli frontend for emerge, plus some extra features. Why? Because I can.\n\nNote: Package operations require root.\nSo run with your preferred method of privilege elevation, otherwise emerge will ask if you want to pretend.")
        .arg(Arg::with_name("clean")
			 .short("c")
			 .long("clean")
			 .help("Remove any un-needed packages."))
        .arg(Arg::with_name("deps")
			 .short("d")
			 .long("deps")
			 .help("View the dependencies of a package. Note: Requires gentoolkit to be installed.")
			 .value_name("PKG")
			 .takes_value(true))
        .arg(Arg::with_name("files")
			 .short("F")
			 .long("files")
			 .help("View the files of a package. Note: Requires gentoolkit to be installed.")
			 .value_name("PKGS")
			 .takes_value(true))
        .arg(Arg::with_name("frem")
			 .short("f")
			 .long("frem")
			 .help("Force remove package(s).")
			 .value_name("PKGS")
			 .takes_value(true)
			 .multiple(true))
		.arg(Arg::with_name("install")
			 .short("i")
			 .long("install")
			 .help("Install package(s).")
		     .value_name("PKGS")
		     .takes_value(true)
		     .multiple(true))
		.arg(Arg::with_name("list")
			 .short("l")
			 .long("list")
			 .help("List currently installed packages."))
        .arg(Arg::with_name("meta")
			 .short("m")
			 .long("metadata")
			 .help("View the metadata of a package. Note: Requires gentoolkit to be installed.")
			 .value_name("PKG")
			 .takes_value(true))
		.arg(Arg::with_name("portup")
			 .short("p")
			 .long("portup")
			 .help("One-shot emerge portage. Used when you need to specifically update portage."))
		.arg(Arg::with_name("remove")
			 .short("r")
			 .long("remove")
			 .help("Remove package(s).")
		     .value_name("PKGS")
		     .takes_value(true)
		     .multiple(true))
		.arg(Arg::with_name("search")
			 .short("s")
			 .long("search")
			 .help("Search for package(s).")
		     .value_name("SEARCHES")
		     .takes_value(true)
		     .multiple(true))
		.arg(Arg::with_name("update")
			 .short("u")
			 .long("update")
			 .help("Update any installed packages."))
        .arg(Arg::with_name("useflags")
			 .short("U")
			 .long("useflags")
			 .help("View the USE flags of a package. Note: Requires gentoolkit to be installed.")
			 .value_name("PKG")
			 .takes_value(true))
		.arg(Arg::with_name("sync")
			 .short("S")
			 .long("sync")
			 .help("Update repos and overlays."))
		.arg(Arg::with_name("world")
			 .short("w")
			 .long("world")
			 .help("Prints the contents of your world file."))
		.get_matches();
    if matches.is_present("clean") {
        clean();
    }
    if let Some(in_deps) = matches.values_of("deps") {
        for d in in_deps {
            deps(d);
        }
    }
    if let Some(in_files) = matches.values_of("files") {
        for F in in_files {
            files(F);
        }
    }
    if let Some(in_frem) = matches.values_of("frem") {
        for f in in_frem {
            frem(f);
        }
    }
    if let Some(in_install) = matches.values_of("install") {
        for i in in_install {
            install(i);
        }
    }
    if matches.is_present("list") {
        list();
    }
    if let Some(in_meta) = matches.values_of("meta") {
        for m in in_meta {
            meta(m);
        }
    }
    if matches.is_present("portup") {
        portup();
    }
    if let Some(in_remove) = matches.values_of("remove") {
        for r in in_remove {
            remove(r);
        }
    }
    if let Some(in_search) = matches.values_of("search") {
        for s in in_search {
            search(s);
        }
    }
    if matches.is_present("sync") {
        sync();
    }
    if matches.is_present("update") {
        update();
    }
    if let Some(in_useflags) = matches.values_of("useflags") {
        for U in in_useflags {
            useflags(U);
        }
    }
    if matches.is_present("world") {
        world();
    }
}
