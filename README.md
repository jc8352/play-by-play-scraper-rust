# Play-by-Play Scraper

This is a concurrent web scraper that retrieves play-by-play data from basketball-reference.com for all games played on a specified date.

## Features

- Concurrent scraping for efficiency
- Retrieves detailed play-by-play data
- Customizable for different dates

## Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

## Usage

1. Clone the repository:
    ```sh
    git clone https://github.com/jc8352/play-by-play-scraper-rust.git
    ```

2. Build the project:
    ```sh
    cargo build
    ```
3. Run the scraper with a specified date:
    ```sh
    cargo run -- MM DD YYYY
    ```

    Replace `MM DD YYYY` with the desired date (e.g. `01 15 2023`, or `1 15 2023`).
    
    The play-by-play will be output to CSV files for each game played on that date.
