use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use regex::{Captures, Regex};
use lazy_static::lazy_static;

fn print_tag_file(tag: &str) -> std::io::Result<()> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[^\t]*").unwrap();
    }

    let file = match File::open(tag) {
        Ok(file) => file,
        // ignore not-exist file
        Err(_) => return Ok(()),
    };
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let content = line?;

        if content.starts_with('!') {
            continue;
        }

        let modified_content = RE.replace(&content, |caps: &Captures| {
            format!("{:-24}", &caps[0])
        });

        println!("{}\t{}", modified_content, tag);
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    for arg in env::args().skip(1) {
        print_tag_file(&arg)?;
    }

    Ok(())
}
