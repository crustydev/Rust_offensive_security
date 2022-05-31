# Crusty Scanner

This is a tool for mapping attack surfaces with subdomain enumeration. It is obtained from the book Black Hat Rust by Sylvain Kerkour.

## Description
Crusty scanner is an asynchronous program designed in Rust to accept a target domain and find its associated subdomains by retrieving entries from a [certificate search website](https://crt.sh), and find out which ports are open for connection in each subdomain.

## Usage

Download the source code, make sure to have both rustc and cargo installed and run in terminal with:  
`cargo run crusty_scan <name.com>`

for example:  
`cargo run crusty_scan <jsattorney.com>`

It displays a list of subdomains associated with our target and displays the open ports for each subdomain.

