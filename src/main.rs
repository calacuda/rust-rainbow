use clap::{App, Arg, ArgMatches};
use std::fs::{create_dir, File};
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
// use std::path::Path;

mod hash;

fn main() {
    let base_dir = "tables";
    let algos = ["md5", "sha1", "sha256", "sha512"]; //, "ntlm_v2"];
    match create_dir(base_dir) {
        _ => {}
    };
    // for algo in algos {
    //     create_dir(format!("{}/{}_table", base_dir, algo));
    // }
    let args = get_args();

    for algo in algos {
        //makes the table if it doesn't exist.
        let current_dir = format!("{}/{}_table", base_dir, algo);
        match create_dir(&current_dir) {
            _ => {}
        };

        // read file into buffer
        let file = match File::open(args.get_one::<String>("wordlist").unwrap()) {
            Ok(file) => file,
            Err(_) => panic!("word list file not found in file system"),
        };
        let reader = BufReader::new(&file);
        println!("algorithm :  {}", &algo);
        let mut i = 1;
        // itterate over lines
        for line in reader.lines() {
            // println!("{}: {:?}", i, line);
            let passwd = match line {
                Ok(word) => word.into_bytes(),
                Err(_) => {
                    println!("failed to parse line {}", i);
                    break;
                }
            };
            let hash_val = hash::hash(algo, &passwd);
            // println!("{:#?}", hash_val);
            // write to file
            let hash_fname = &format!("{}/{}", &current_dir, hash_val);
            // let path = Path::new(&hash_fname);

            let mut hash_file = match File::create(hash_fname) {
                Ok(f) => f,
                Err(_) => {
                    println!("failed to make file: {}!", hash_fname);
                    break;
                }
            };
            match hash_file.write_all(&passwd) {
                Ok(_) => {}
                Err(_) => println!("could not write line"),
            };
            i += 1;
        }
    }
}

fn get_args() -> ArgMatches {
    return App::new("rust-rainbow")
        .version("0.0.1")
        .author("Calacuda. <https://github.com/calacuda>")
        .about("used to generate a rainbow table based on a wordlist")
        .arg(
            Arg::new("wordlist")
                .short('w')
                .long("wordlist")
                .value_name("wordlist.txt")
                .help("The wordlist that will be used to generate the table.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
}
