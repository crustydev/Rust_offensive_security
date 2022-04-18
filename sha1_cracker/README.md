# SHA1 cracker 

This program tries to take in a sha1 hash and return its original content.

## Description
Sha1 cracker is a program designed to take a .txt filename and 40-character length sha1 hash as arguments, applying brute-force search to match the hash with a word in the .txt file and thus recover the original word if it's present in the file. It draws inspiration from the book **Black Hat Rust** by **Silvain Keykour**.

## Usage

It can simply be run by: 
`cargo run <wordlist.txt> <sha1_hash>`
for example
`cargo run wordlist.txt 22d7a281973a39b8fafac353e4225c7ff98e99cc`

You also can generate test_cases and run them to see for yourself if the cracker works using:
`cargo run create_test_hashes` and following the steps.
This creates a _hashtest.json_ file that maps each of your string inputs to its valid sha1 hash. You can then use:
`cargo run tests`. 

If the program works, creating some test cases from words you know to be present in the wordlist and others from ones you know aren't in the wordlist should give results as expected.
