use serde_json::{json, value::Value, Map};
use sha1::Digest;
use std::io;
use std::{
    env,
    error::Error,
    fs,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        if args.len() == 2 {
            if &args[1] == "create_test_hashes" {
                return create_test_hashes();
            } else if &args[1] == "tests" {
                return run_test();
            }
        }
        println!("\nInvalid command, run binary either in this format: \n");
        println!("<cargo run create_test_hashes> (to create test instances) and then");
        println!("<cargo run test> (to run tests) \n");
        println!("or this format: \n");
        println!("<cargo run 'wordlist' 'sha1_hash'> ");

        return Ok(());
    }

    let hash_to_crack = &args[2].trim();

    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash not valid".into());
    }

    return find_word_from_hash(hash_to_crack, &args[1]);
}

fn find_word_from_hash(hash: &str, words_filename: &str) -> Result<(), Box<dyn Error>> {
    if hash.len() != SHA1_HEX_STRING_LENGTH {
        return Err("Invalid sha1 hash".into());
    }

    let wordlist_file =
        File::open(words_filename).expect(&format!("error opening {}", &words_filename));
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line.expect(&format!("error reading line in {}", &words_filename));
        let password = line.trim();
        if hash == &hex::encode(sha1::Sha1::digest(&password.as_bytes())) {
            println!("Password found: {}\n", &password);
            return Ok(());
        }
    }

    println!("password not found in {}\n", &words_filename);
    Ok(())
}

// Creates test hashes. Takes input from the user and hashes it.
// It returns a <password> ; <hash> in json format to a hash_tests.json

fn create_test_hashes() -> Result<(), Box<dyn Error>> {
    let mut hash_tests: Map<String, Value> = Map::new();

    println!("Starting hashtest file generator...");
    println!("Enter words you know are present in wordlist");
    println!("and others you know likely aren't present\n");

    loop {
        println!("Enter word to be hashed, 'done' to end ");
        let mut password = String::new();

        io::stdin().read_line(&mut password).expect("Invalid input");
        password = password.trim().to_string();

        if &password == "done" {
            break;
        }

        let hash = &hex::encode(sha1::Sha1::digest(&password.as_bytes()));

        println!("word: {}... hash: {}\n", &password, &hash);
        hash_tests.insert(password.clone(), json!(hash));
    }

    let test_json = json!(hash_tests);

    if Path::new("hashtest.json").exists() == false {
        File::create("hashtest.json").expect("Error opening hashtest.json");
    }

    fs::write(String::from("hashtest.json"), test_json.to_string())
        .expect("Unable to write to hashtest.json");

    println!("sample word-hash pairs created and written to hashtest.json successfully!");
    Ok(())
}

fn run_test() -> Result<(), Box<dyn Error>> {
    let hash_test_file = "hashtest.json";
    let wordlist_file = "wordlist.txt";

    let mut tests = File::open(&hash_test_file).unwrap();
    let mut data = String::new();
    tests.read_to_string(&mut data).unwrap();

    let json: Value = serde_json::from_str(&data).unwrap();
    let hashes: Map<String, Value> = json.as_object().unwrap().clone();

    for (_, hash) in hashes {
        let hash_to_crack = String::from(hash.as_str().unwrap());
        println!("hash: {}", &hash_to_crack);
        find_word_from_hash(&hash_to_crack, &wordlist_file).unwrap();
    }

    println!("Tests completed successfully!");
    Ok(())
}
