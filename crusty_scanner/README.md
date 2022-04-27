# Crusty scanner

This is a tool for mapping attack surfaces using subdomain enumeration.

## Description
Crusty scanner is an asynchronous program designed in Rust to accept a target domain and find its associated subdomains by retrieving entries from a [certificate search website](https://crt.sh), and find out what ports are open for connection in each subdomain.

## Usage

To run, download the source code, making sure to have rustc and cargo installed, and type:
`cargo run crusty_scan <name.com>` in your terminal to run.

for example:
`cargo run crusty_scan <jsattorney.com>`

If it works as it's meant to, there'll be a list of associated subdomains with your target and the ports they have open.