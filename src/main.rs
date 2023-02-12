use std::{fs::{self, File}, io::{Read, Error}, time::Instant};
use regex::Regex;
use walkdir::WalkDir;
use num_format::{Locale, ToFormattedString};

fn main() -> Result<(), Error> {
    // start timer
    let n = Instant::now();
    // constant base path
    const PATH: &str = "/home";
    // instantiate counters
    let mut file_count: u64 = 0;
    let mut dir_count: u64 = 0;
    let mut valid_count: u64 = 0;
    let mut match_count: u32 = 0;
    let mut err_count: u32 = 0;
    // regex
    let mn_24: Regex = Regex::new(r"^(\w+\s){23}\w+$").unwrap();
    let mn_12: Regex = Regex::new(r"^(\w+\s){11}\w+$").unwrap();
    let so_88: Regex = Regex::new(r"^[a-zA-Z0-9_]{88}$").unwrap();
    let so_64: Regex = Regex::new(r"^\[([0-9]+,){63}[0-9]+\]$").unwrap();
    let mm_32: Regex = Regex::new(r"^[a-z0-9_]{32}$").unwrap();
    let mm_64: Regex = Regex::new(r"^[a-z0-9_]{64}$").unwrap();
    // result vector
    let mut results: Vec<String> = Vec::new();
    // start recursive walker
    for entry in WalkDir::new(PATH).follow_links(true).into_iter().filter_map(|entry| entry.ok()) {
        // get path from entry
        let entry_path = entry.path();
        let display_path = entry_path.display();

        println!("{}", display_path);
        // get metadata to differentiate files & dirs
        let meta = fs::metadata(&entry_path)?;
        // we ignore directories but count them
        if meta.is_dir() {
            dir_count += 1;
        }
        // runs only if entry is a file
        if meta.is_file() {
            file_count += 1;
            // Create a path to the desired file
            // Open the path in read-only mode, returns `io::Result<File>`
            if let b = std::path::Path::new(&entry_path).exists() {
                let _file = match File::open(&entry_path) {
                    Err(_) => {
                        err_count += 1;
                    },
                    Ok(mut _file) => {
                        // instantiate vector only if we can open the file
                        let mut content: Vec<u8> = Vec::new();
                        // ingest bytes data
                        if _file.read_to_end(&mut content).is_ok() {
                            // past here, we have a utf-8 readable file
                            if std::str::from_utf8(&content).is_ok() {
                                // increment read_count
                                valid_count += 1;
                                // read file
                                let res: String = String::from(std::str::from_utf8(&content).unwrap());
                                // regex assertion
                                if (mn_12.is_match(&res) || mn_24.is_match(&res) || so_88.is_match(&res) || so_64.is_match(&res) || mm_32.is_match(&res) || mm_64.is_match(&res)) == true {
                                    match_count += 1;
                                    println!("{}", &res);
                                    println!("{}", &display_path);
                                    results.push(res);
                                }
                            };
                        }
                        
                    },
                };
            }
            
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
        "Found {} files in {} directories\nRead {} files, {} matches in {:.2}s\nEncountered {} Errors.",
        file_count.to_formatted_string(&Locale::en), 
        dir_count.to_formatted_string(&Locale::en), 
        valid_count.to_formatted_string(&Locale::en), 
        match_count.to_formatted_string(&Locale::en),
        n,
        err_count.to_formatted_string(&Locale::en),
    );
    Ok(())
}