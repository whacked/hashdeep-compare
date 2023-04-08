use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;


/** 
 * the hashdeep datastructure
 */
struct HashdeepLine {
    size: String,
    md5: String,
    sha256: String,
    filename: String,
}

impl HashdeepLine {
    fn new(line: &str, fields: &Vec<String>) -> HashdeepLine {
        let mut hashdeep_line = HashdeepLine {
            size: String::new(),
            md5: String::new(),
            sha256: String::new(),
            filename: String::new(),
        };
        let mut line_iter = line.splitn(fields.len(), ",").map(|s| s.to_string());
        // assign fields to the struct in the order given in fields
        for field in fields {
            if field == "size" {
                hashdeep_line.size = line_iter.next().unwrap().to_string();
            } else if field == "md5" {
                hashdeep_line.md5 = line_iter.next().unwrap().to_string();
            } else if field == "sha256" {
                hashdeep_line.sha256 = line_iter.next().unwrap().to_string();
            } else if field == "filename" {
                hashdeep_line.filename = line_iter.next().unwrap().to_string();
            }
        }
        hashdeep_line
    }
}

/**
 * this function takes the path of a text file
 * that is expected to be the output of hashdeep.
 * for each line, it reads the hashdeep output,
 * and parses the line into the datastructure
 */
fn read_hashdeep_file_to_vector(path: &str) -> Vec<HashdeepLine> {
    let mut hashdeep_lines: Vec<HashdeepLine> = Vec::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut fields: Vec<String> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        // checks on the first 5 lines to ensure this is a hashdeep file
        if index == 0 {
            if !line.starts_with("%%%% HASHDEEP-1.0") {
                println!("{} is not a hashdeep file: line 1 needs to start with hashdeep header", path);
                std::process::exit(1);
            }
        }  else if index == 1 {
            if !line.starts_with("%%%% size,md5") {
                println!("{} is not a hashdeep file: line 2 must be hashdeep format header", path);
                std::process::exit(1);
            }
            // remove the %%%%
            fields = line.trim_start_matches("%%%% ").split(",").map(|s| s.to_string()).collect();
        } else if index == 2 {
            // line must contain invocation path
            if !line.starts_with("## Invoked from: ") {
                println!("{} is not a hashdeep file: line 3 must contain invocation path; got: {}", path, line);
                std::process::exit(1);
            }
        } else if index == 3 {
            // line must contain invocation command
            if !line.starts_with("## $ hashdeep ") {
                println!("{} is not a hashdeep file: line 4 must contain hashdeep command", path);
                std::process::exit(1);
            }
        } else if index == 4 {
            // line must contain ##
            if !line.starts_with("## ") {
                println!("{} is not a hashdeep file: line 5 must be spacer", path);
                std::process::exit(1);
            }
        } else {
            let hashdeep_line = HashdeepLine::new(&line, &fields);
            hashdeep_lines.push(hashdeep_line);
        }
    }
    hashdeep_lines
}

/**
 * datastructure that stores summary statistics from compare.
 * it keeps tracks of these totals:
 * - not in base
 * - different size
 * - in base, same size
 */
struct CompareStats {
    not_in_base: u32,
    different_size: u32,
    same_size: u32,
}

/**
 * this function takes 2 hashmaps of HashdeepLine records
 * and performs a forward comparison of their contents
 */
fn compare_hashdeep_hashmaps(base_map: &HashMap<String, HashdeepLine>, comp_map: &HashMap<String, HashdeepLine>) -> Result<CompareStats, String> {
    let mut summary_stats = CompareStats {
        not_in_base: 0,
        different_size: 0,
        same_size: 0,
    };
    let padding_width = comp_map.len().to_string().len();
    // interate through comp_map with an index
    // and check if the hash exists in base_map
    // if it does, check if the size is the same
    // and print out whether the test succeeded
    for (index, (comp_hash, comp_record)) in comp_map.iter().enumerate() {
        if index < 5 {
            println!("SAMPLE {}: {} {}", index, &comp_hash[0..8], comp_record.filename);
        }
        if !base_map.contains_key(comp_hash) {
            println!("{:width$}: {} {} {}", index, "NOT IN SOURCE:", &comp_hash[0..8], comp_record.filename, width=padding_width);
            summary_stats.not_in_base += 1;
        } else {
            let base_record = base_map.get(comp_hash).unwrap();
            if base_record.size == comp_record.size {
                // println!("{:width$}: {} {} {}", index, &comp_hash[0..8], comp_record.filename, "OK", width=padding_width);
                summary_stats.same_size += 1;
            } else {
                summary_stats.different_size += 1;
            }
        }
    }

    println!("\ndone; compared {} records", comp_map.len());
    // print the summary stats
    println!("{} records not in source", summary_stats.not_in_base);
    println!("{} records different size", summary_stats.different_size);
    println!("{} records equivalent", summary_stats.same_size);

    return Ok(summary_stats);
}

/**
 * this function takes a vector of HashdeepLine records
 * and returns a HashMap where the keys are the hash
 * and the values are the records
 */
fn hashdeep_lines_to_hashmap(hashdeep_lines: Vec<HashdeepLine>) -> HashMap<String, HashdeepLine> {
    let mut hashdeep_hashmap: HashMap<String, HashdeepLine> = HashMap::new();
    for hashdeep_line in hashdeep_lines {
        hashdeep_hashmap.insert(hashdeep_line.md5.clone(), hashdeep_line);
    }
    hashdeep_hashmap
}

fn main() {
    // read command line arguments and print them to stdout
    for arg in std::env::args() {
        println!("{}", arg);
    }

    // assert that there are at least 2 stdin arguments
    if std::env::args().len() < 3 {
        println!("Usage: {} <base_file> <comp_file>", std::env::args().nth(0).unwrap());
        std::process::exit(1);
    }
    
    // assign the first command line argument to a variable
    // that stores the path to the first input file
    let base_file = std::env::args().nth(1).unwrap();
    // assign the second command line argument to a variable
    // that stores the path to the second input file
    let comp_file = std::env::args().nth(2).unwrap();

    let reversed = if std::env::args().len() > 3 {
        std::env::args().nth(3).unwrap().starts_with("--reverse")
    } else {
        false
    };

    println!("hashdeep-compare running for\n- {}\n- {}", base_file, comp_file);

    // bail if the input files don't exist
    if !Path::new(&base_file).exists() {
        println!("{} does not exist", base_file);
        std::process::exit(1);
    }
    if !Path::new(&comp_file).exists() {
        println!("{} does not exist", comp_file);
        std::process::exit(1);
    }

    // load the files
    let base_file_records = read_hashdeep_file_to_vector(&base_file);
    let comp_file_records = read_hashdeep_file_to_vector(&comp_file);

    // print summary stats for both record arrays
    println!("{} records in {}", base_file_records.len(), base_file);
    println!("{} records in {}", comp_file_records.len(), comp_file);

    let base_map = hashdeep_lines_to_hashmap(base_file_records);
    let comp_map = hashdeep_lines_to_hashmap(comp_file_records);

    if reversed {
        println!("RUNNING REVERSED DIRECTION");
        println!("now checking whether all keys from\n(SOURCE) {} exist in\n(TARGET) {}...", base_file, comp_file);
        compare_hashdeep_hashmaps(&comp_map, &base_map);
    } else {
        println!("now checking whether all keys from\n(TARGET) {} exist in\n(SOURCE) {}...", comp_file, base_file);
        compare_hashdeep_hashmaps(&base_map, &comp_map);
    }

    println!("main function complete");
}
