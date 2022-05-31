# Crawler

This is a tool for crawling websites and scraping data. It is obtained from the book Black Hat Rust by Sylvain Kerkour.

## Description
This is a tool designed in Rust to crawl website according to its different spiders and their functionality. Currently it has the cvedetails spider for retrieving cve information from cvedetails.com, the github spider for retrieving members of a github organization from github.com, and the quotes spider for retrieving quotes from a javascript web application.

## Usage

Download the source code, make sure to have both rustc and cargo installed and run in terminal with:  
`cargo run crawl <spider>`

for example:  
`cargo run crawl github`

Use `cargo run spiders` to list available spiders.

