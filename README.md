# Contact info scraper

This program will read `website_list.csv` file and parse all website links. After that it will attempt to find contact page of those websites and then extract all emails present.

## Try it out!

Install [Rust](https://rustup.rs/), edit `website_list.csv` with your websites and run:
```bash
$ cargo run
```

The program will print the contact pages it found and then save all emails to `contact_list.csv`.
