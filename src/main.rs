extern crate clap;
extern crate glob;
use clap::{Arg, App};
use glob::glob;
use std::process::Command;

fn main() {
    let matches = App::new("pkg")
		.version("0.0.1")
		.author("Phate6660 <https://github.com/Phate6660>")
		.about("A cli frontend for emerge. Why? Because I can.")
        .arg(Arg::with_name("install")
			 .short("i")
			 .long("install")
			 .help("Install a package. Note: It automatically confirms the installation.")
		     .value_name("PKGS")
		     .takes_value(true)
		     .multiple(true))
		.arg(Arg::with_name("search")
			 .short("s")
			 .long("search")
			 .help("Search for a package.")
		     .value_name("SEARCHES")
		     .takes_value(true)
		     .multiple(true))
		.arg(Arg::with_name("list")
			 .short("l")
			 .long("list")
			 .help("List currently installed packages."))
		.get_matches();
	if matches.is_present("list") {
		for entry in glob("/var/db/pkg/*/*/").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => println!("{:?}", path.display()),
                Err(e) => println!("{:?}", e),
            }
        }
	}
	if let Some(in_search) = matches.values_of("search") {
        for s in in_search {
		    let results = Command::new("emerge")
			    .arg("-s")
			    .arg(s)
			    .output()
			    .expect("Failed to search for the package.");
		    println!("{}", String::from_utf8_lossy(&results.stdout));
		}
    }
	if let Some(in_install) = matches.values_of("install") {
        for i in in_install {
		    Command::new("emerge")
			    .arg("-t")
			    .arg("-v")
			    .arg(i)
			    .spawn()
			    .expect("Failed to install the package.");
		}
    }
}
