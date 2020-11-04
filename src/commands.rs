use glob::glob;
use sedregex::find_and_replace;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn clean() {
    let child = Command::new("emerge")
        .args(&["-a", "-D", "-c"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Could not clean system of un-needed packages.");
    io::stdout().write_all(&child.stdout).unwrap();
}

#[cfg(feature = "gentoolkit")]
pub fn deps(d: &str) {
    let child = Command::new("equery")
        .args(&["g", d])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to display the deps, is gentoolkit installed?");
    io::stdout().write_all(&child.stdout).unwrap();
}

#[cfg(not(feature = "gentoolkit"))]
pub fn deps(d: &str) {
    println!("Can not list deps of {}, you did not enable the gentoolkit feature.", d);
}

#[cfg(feature = "gentoolkit")]
pub fn files(F: &str) {
    let child = Command::new("equery")
        .args(&["f", F])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to display the files in the package, is gentoolkit installed?");
    io::stdout().write_all(&child.stdout).unwrap();
}

#[cfg(not(feature = "gentoolkit"))]
pub fn files(F: &str) {
    println!("Can not list the files of {}, you did not enable the gentoolkit feature.", F);
}

pub fn frem(f: &str) {
    let child = Command::new("emerge")
        .args(&["-a", "-v", "-C", f])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to install the package(s).");
    io::stdout().write_all(&child.stdout).unwrap();
}

pub fn install(i: &str) {
    let child = Command::new("emerge")
        .args(&["-a", "-t", "-v", i])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to install the package(s).");
    io::stdout().write_all(&child.stdout).unwrap();
}

pub fn list() {
    for entry in glob("/var/db/pkg/*/*/").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let message = find_and_replace(&path.display().to_string(), &["s/\"//g"])
                    .unwrap()
                    .to_string();
                let content: Vec<&str> = message.split('/').collect();
                let mut terminal = term::stdout().unwrap();
                terminal.attr(term::Attr::Bold).unwrap();
                terminal.fg(term::color::WHITE).unwrap();
                write!(terminal, "{}/", content[4])
                    .expect("Could not write the package category.");
                terminal.fg(term::color::YELLOW).unwrap();
                writeln!(terminal, "{}", content[5])
                    .expect("Could not write the package name and version.");
                terminal.reset().unwrap();
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

#[cfg(feature = "gentoolkit")]
pub fn meta(m: &str) {
    let child = Command::new("equery")
        .args(&["m", m])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to display the metadata, is gentoolkit installed?");
    io::stdout().write_all(&child.stdout).unwrap();
}

#[cfg(not(feature = "gentoolkit"))]
pub fn meta(m: &str) {
    println!("Can not list metadata of {}, you did not enable the gentoolkit feature.", m);
}

pub fn portup() {
    let child = Command::new("emerge")
        .args(&["-a", "-1", "sys-apps/portage"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to update Portage.");
    io::stdout().write_all(&child.stdout).unwrap();
}

pub fn remove(r: &str) {
    let child = Command::new("emerge")
        .args(&["-a", "-v", "-c", r])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to remove the package(s).");
    io::stdout().write_all(&child.stdout).unwrap();
}

pub fn search(s: &str) {
    let child = Command::new("emerge")
        .args(&["-s", s])
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to search for the package(s).");
    io::stdout().write_all(&child.stdout).unwrap();
}

pub fn sync() {
    let child = Command::new("emerge")
        .arg("--sync")
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to sync.");
    io::stdout().write_all(&child.stdout).unwrap();
}

pub fn update() {
    let child = Command::new("emerge")
        .args(&["-a", "-v", "-u", "-D", "-N", "--with-bdeps", "y", "@world"])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to update packages.");
    io::stdout().write_all(&child.stdout).unwrap();
}

#[cfg(feature = "gentoolkit")]
pub fn useflags(U: &str) {
    let child = Command::new("equery")
        .args(&["u", U])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to display the USE flags, is gentoolkit installed?");
    io::stdout().write_all(&child.stdout).unwrap();
}

#[cfg(not(feature = "gentoolkit"))]
pub fn useflags(u: &str) {
    println!("Can not list the USE flags of {}, you did not enable the gentoolkit feature.", u);
}

pub fn world() {
    let mut file = File::open("/var/lib/portage/world")
        .expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    println!("{}", contents.trim());
}
