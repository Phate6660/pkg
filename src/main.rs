extern crate clap;
extern crate glob;
use clap::{Arg, App};
use glob::glob;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::process::{Command,Stdio};

fn main() {
    let matches = App::new("pkg")
		.version("0.0.1")
		.author("Phate6660 <https://github.com/Phate6660>")
		.about("A cli frontend for emerge, plus some extra features. Why? Because I can.")
        .arg(Arg::with_name("clean")
			 .short("c")
			 .long("clean")
			 .help("Remove any un-needed packages."))
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
		.arg(Arg::with_name("portup")
			 .short("p")
			 .long("portup")
			 .help("One-shot emerge portage. Used when you need to specifically update porage."))
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
		let child = Command::new("emerge")
			.arg("-a")
			.arg("-D")
			.arg("-c")
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.output()
			.expect("Could not clean system of un-needed packages.");
		io::stdout().write_all(&child.stdout).unwrap();
	}
	if let Some(in_frem) = matches.values_of("frem") {
        for f in in_frem {
		    let child = Command::new("emerge")
				.arg("-a")
			    .arg("-v")
			    .arg("-C")
			    .arg(f)
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
			    .output()
			    .expect("Failed to install the package(s).");
			io::stdout().write_all(&child.stdout).unwrap();
		}
    }
	if let Some(in_install) = matches.values_of("install") {
        for i in in_install {
		    let child = Command::new("emerge")
				.arg("-a")
			    .arg("-t")
			    .arg("-v")
			    .arg(i)
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
			    .output()
			    .expect("Failed to install the package(s).");
			io::stdout().write_all(&child.stdout).unwrap();
		}
    }
	if matches.is_present("list") {
		for entry in glob("/var/db/pkg/*/*/").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("{:?}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
	}
	if matches.is_present("portup") {
		let child = Command::new("emerge")
			.arg("-a")
			.arg("-1")
			.arg("sys-apps/portage")
			.stdin(Stdio::inherit())
			.stdout(Stdio::inherit())
			.output()
			.expect("Failed to update Portage.");
		io::stdout().write_all(&child.stdout).unwrap();
	}
	if let Some(in_remove) = matches.values_of("remove") {
        for r in in_remove {
		    let child = Command::new("emerge")
				.arg("-a")
			    .arg("-v")
			    .arg("-c")
			    .arg(r)
				.stdin(Stdio::inherit())
				.stdout(Stdio::inherit())
			    .output()
			    .expect("Failed to remove the package(s).");
			io::stdout().write_all(&child.stdout).unwrap();
		}
    }
	if let Some(in_search) = matches.values_of("search") {
        for s in in_search {
		    let results = Command::new("emerge")
			    .arg("-s")
			    .arg(s)
			    .output()
			    .expect("Failed to search for the package(s).");
		    println!("{}", String::from_utf8_lossy(&results.stdout));
		}
    }
	if matches.is_present("sync") {
		Command::new("emerge")
			    .arg("--sync")
			    .spawn()
			    .expect("Failed to sync.");
	}
	if matches.is_present("world") {
		let mut file = File::open("/var/lib/portage/world").expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");
        println!("{}", contents);
	}
}
