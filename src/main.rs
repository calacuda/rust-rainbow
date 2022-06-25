use clap::{App, Arg, ArgMatches};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::fs::{create_dir, File};
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process;
// use std::io::prelude::*;

mod hash;

fn main() {
    let args = get_args();

    match args.try_contains_id("list") {
        Ok(is_there) => {
            // println!("{:?}", args);
            if is_there {
                println!(
                    "suported algorithms :  {}",
                    args.get_one::<String>("positive_algols").unwrap(),
                );
                process::exit(0);
            }
        }
        Err(e) => {
            println!("{}", e)
        }
    }

    let wordlist_f = Path::new(args.get_one::<String>("wordlist").unwrap());

    if !wordlist_f.exists() {
        println!("word list file not found in file system");
        process::exit(1);
    }

    let base_dir = &format!(
        "{}_tables",
        wordlist_f.file_stem().unwrap().to_str().unwrap()
    );

    match create_dir(base_dir) {
        _ => {}
    };

    let mut algos = args
        .get_one::<String>("positive_algols")
        .unwrap()
        .split(",")
        .collect::<HashSet<&str>>();

    match args.get_one::<String>("negative_algols") {
        Some(neg_algos) => {
            for no_al in neg_algos.split(",") {
                algos.remove(&no_al);
            }
        }
        None => {}
    }

    match args.get_one::<String>("type").unwrap().as_str() {
        "file" => {
            println!("[LOG] :  making a file based database.");
            file_table(&algos, &base_dir, &args);
        }
        "dir" => {
            println!("[LOG] :  making a directory based database.");
            dir_table(&algos, &base_dir, &args);
        }
        _ => {
            println!(
                "[ERROR] :  the \"type\" comand line argument can't be \"{}\", \
                it can only be \"file\" or \"dir\"",
                args.get_one::<String>("type").unwrap()
            );
            process::exit(1);
        }
    }

    println!("I've finished computing, good luck, and happy password cracking!")
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
                .value_name("WORDLIST-FILE")
                .help("The wordlist that will be used to generate the table.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help(
                    "The type of database to build. Options: file or dir, \
                    (Default: file) the \"file\" option will output to a txt \
                    file where each line is <HASH>:<PASSWORD>. The \"dir\" \
                    option will build a directory structure in the format of, \
                    tables/<ALGORITHM>_table/<HASH>, The contents of the file \
                    <HASH> is the coresponding password. The file option takes \
                    MUCH less space but is a slower search, the dir is a MUCH \
                    larger table but gives faster searches. pick your poison.",
                )
                .takes_value(true)
                .required(false)
                .default_value("file"),
        )
        .arg(
            Arg::new("positive_algols")
                .short('a')
                .long("algorithms")
                .value_name("ALGORITHM_1,ALGORITHM_2")
                .help(
                    "a comma separated list of the algorithms to use. \
                    (default is all: \"-a md5,sha1,sha256,sha512,ntlm_v2\")",
                )
                .takes_value(true)
                .required(false)
                .default_value("md5,sha1,sha256,sha512,ntlm_v2"),
        )
        .arg(
            Arg::new("negative_algols")
                .short('s')
                .long("skip-algorithms")
                .value_name("ALGORITHM_1,ALGORITHM_2")
                .help("a comma separated list of the algorithms to not use.")
                .takes_value(true)
                .required(false), // .default_value(","),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .help("lists suported algorithms")
                .exclusive(true)
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::new("line-buffer")
                .short('b')
                .long("line-buffer")
                .help(
                    "the size of the line buffer, bugger numbers are faster. \
                    however when increasing the line buffer size, one will reach \
                    a point of diminishing returns. the max value is a unisgned \
                    16-bit int.",
                )
                .takes_value(true)
                .required(false)
                .default_value("50"),
        )
        .get_matches();
}

fn file_table(algos: &HashSet<&str>, base_dir: &str, args: &ArgMatches) {
    let line_buf = match args.get_one::<String>("line-buffer") {
        Some(num) => match num.to_owned().parse::<u16>() {
            Ok(n) => n,
            Err(_) => {
                println!("[Error] :  could not interpret \"{}\" as an u16!", num);
                process::exit(1);
            }
        },
        None => 50,
    };

    for algo in algos {
        println!("hash algorithm :  {}", &algo);

        //makes the table if it doesn't exist.
        let db_fname = format!("{}/{}_table.txt", base_dir, algo);

        // if the database already exists LEAVE IT ALONE! DON'T DELETE IT,
        if Path::new(&db_fname).exists() {
            // move on to next algorithm
            continue;
        }

        // read file into buffer
        let file = match File::open(args.get_one::<String>("wordlist").unwrap()) {
            Ok(file) => file,
            Err(_) => {
                println!("word list file not found in file system");
                process::exit(1);
            }
        };

        let mut reader = BufReader::new(&file);
        match File::create(&db_fname) {
            Ok(f) => f,
            Err(_) => {
                println!("failed to make file: {}!", &db_fname);
                continue;
            }
        };

        let mut db_file = OpenOptions::new()
            .append(true)
            .open(&db_fname)
            .expect(&format!("Unable to open file named: {}", &db_fname));

        // itterate over lines
        loop {
            let mut write_buf: Vec<u8> = Vec::new();
            let mut done: bool = false;

            for _ in 0..line_buf {
                let mut line: Vec<u8> = Vec::new();
                match reader.read_until('\n' as u8, &mut line) {
                    Ok(num) => {
                        if num == 0 {
                            done = true;
                            break;
                        }
                    }
                    Err(_) => break,
                };

                let mut passwd_bytes = line.clone();

                // if passwd_bytes.last() != Some(&('\n' as u8)) {
                //     passwd_bytes.push('\n' as u8);
                //     println!("pushing new line!");
                //     // passwd_bytes.pop();
                // }

                let mut hash_pass: Vec<u8> = hash::hash(algo, &passwd_bytes).as_bytes().to_vec();

                // save to write buffer
                write_buf.append(&mut hash_pass);
                write_buf.push(':' as u8);
                write_buf.append(&mut passwd_bytes);
            }
            match db_file.write_all(&write_buf) {
                Ok(_) => {}
                Err(_) => println!("[ERROR] :  couldn't write password to file: {}", &db_fname),
            };
            if done {
                break;
            }
        }
    }
}

fn dir_table(algos: &HashSet<&str>, base_dir: &str, args: &ArgMatches) {
    // do for once for each algorithm
    for algo in algos {
        //makes the table if it doesn't exist.
        let current_dir = format!("{}/{}_table", base_dir, algo);
        match create_dir(&current_dir) {
            _ => {}
        };

        // read file into buffer
        let file = match File::open(args.get_one::<String>("wordlist").unwrap()) {
            Ok(file) => file,
            Err(_) => {
                println!("word list file not found in file system");
                process::exit(1);
            }
        };
        let reader = BufReader::new(&file);
        println!("algorithm :  {}", &algo);
        let mut i = 1;
        // itterate over lines
        for line in reader.lines() {
            let passwd = match line {
                Ok(word) => word.into_bytes(),
                Err(_) => {
                    println!("failed to parse line {}", i);
                    continue;
                }
            };

            let hash_val = hash::hash(algo, &passwd);
            // write to file
            let hash_fname = &format!("{}/{}", &current_dir, hash_val);

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
