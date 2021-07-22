use std::io::{BufRead, BufReader, Error};
use std::path::Path;
use std::fs::File;
use clap::{Arg, App};

fn main() {
    let matches = App::new("rat")
        .version("0.1.0")
        .author("Daniel Larsen <i@iyyel.io>")
        .about("cat rewritten in Rust")
        .arg(Arg::with_name("FILE")
            .takes_value(true)
            .multiple(true)
            .help("A file whose content will be printed"))
        .arg(Arg::with_name("")
            .takes_value(false)
            .short("n")
            .long("number")
            .help("Number all output lines"))
        .get_matches();

     if let Some(file_args) = matches.values_of("FILE") {
        for file_arg in file_args {
            handle_file_arg(String::from(file_arg));
        }
     } else {
        println!("No file arg provided!")
     }
}

fn handle_file_arg(file_arg: String) {
    if Path::new(&file_arg).exists() {
       match print_file(&file_arg.as_str()) {
           Ok(_) => println!("good!"),
           Err(err) => println!("bad! {}", err)
       };
    } else {
       println!("rat: {}: No such file or directory", file_arg)
    }
}

fn print_file(file_name: &str) -> Result<(), Error> {
    let file = File::open(file_name).unwrap();
    let buf = BufReader::new(file);

    for line in buf.lines() {
          println!("{}", line?);
    }

    Ok(())
}