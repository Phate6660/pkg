extern crate clap;
use clap::{Arg, App};
use std::process::Command;

fn main() {
    let matches = App::new("pkg")
		.version("0.0.1")
		.author("Phate6660 <https://github.com/Phate6660>")
		.about("A cli frontend for emerge. Why? Because I can.")
		.arg(Arg::with_name("search")
			 .short("s")
			 .long("search")
			 .help("Search for a package.")
		     .value_name("SEARCH")
		     .takes_value(true))
		.get_matches();
	if let Some(s) = matches.value_of("search") {
        let results = Command::new("emerge")
			.arg("-s")
			.arg(s)
			.output()
			.expect("Failed to search for the package.");
		println!("{}", String::from_utf8_lossy(&results.stdout));
    }
}
