extern crate uuid;
extern crate docopt;
extern crate rustc_serialize;

use std::convert::AsRef;
use std::fs::{File, remove_file, remove_dir_all, create_dir};
use std::io::prelude::*;
use std::io::{stdin, stdout};
use docopt::Docopt;
use rustc_serialize::json::Json;
use uuid::Uuid;


const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Usage: jsonlite <cmd> <data>
       jsonlite <cmd>
";


#[derive(RustcDecodable)]
struct Args {
    arg_cmd: String,
    arg_data: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                        .and_then(|d| d.decode())
                        .unwrap_or_else(|e| e.exit());

    match args.arg_cmd.as_ref() {
        "set"     => set(&args.arg_data),
        "get"     => get(&args.arg_data),
        "delete"  => delete(&args.arg_data),
        "drop"    => drop(),
        "version" => println!("{}", VERSION),
        "help"    => println!("{}", USAGE),
        _         => println!("Unknown"),
    }
}


fn set(json: &str) {
    let uuid = Uuid::new_v4().to_hyphenated_string().to_uppercase();
    let path = format!("jsonlite.data/{}", uuid);

    create_dir("jsonlite.data").ok();

    let mut file = File::create(&path).unwrap();
    let json = Json::from_str(json).unwrap();

    file.write_all(json.pretty().to_string().as_bytes()).unwrap();

    println!("{}", uuid);
}

fn get(uuid: &str) {
    let path = format!("jsonlite.data/{}", uuid);
    let mut file = File::open(&path).unwrap();
    let mut json = String::new();

    file.read_to_string(&mut json).unwrap();

    println!("{}", json);
}

fn delete(uuid: &str) {
    let path = format!("jsonlite.data/{}", uuid);

    match remove_file(path) {
        Ok(_)  => (),
        Err(e) => panic!("Error deleting {}: {}", uuid, e),
    }
}

fn drop() {
    let path = "jsonlite.data/";

    let mut confirm = String::new();

    print!("Are you sure you want to drop '{}'? (y/n) ", path);
    stdout().flush().ok();
    stdin().read_line(&mut confirm).ok();

    if ["y", "Y", "yes", "YES"].contains(&confirm.trim().as_ref()) {
        remove_dir_all(path).unwrap();
    }
}
