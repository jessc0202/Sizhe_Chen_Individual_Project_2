[![Rust CI/CD Pipeline](https://github.com/jessc0202/Sizhe_Chen_Individual_Project_2/actions/workflows/ci.yml/badge.svg)](https://github.com/jessc0202/Sizhe_Chen_Individual_Project_2/actions/workflows/ci.yml)

# Sizhe Chen Individual Project 2

## Project Overview
This project provides a set of Rust-based tools to interact with and manage a database of candy rankings data. The project includes functionalities to extract, load, and query data from a CSV file into an SQLite database, making it easy to analyze and manipulate candy data.

## Features

- **Data Extraction**: Downloads a CSV file from a specified URL and saves it locally.
- **Transform and Load**: Loads the CSV data into an SQLite database, creating a structured format for querying.
- **Database Queries**: Enables various SQL operations, such as `INSERT`, `UPDATE`, `DELETE`, and `SELECT`, on the candy data within the database.
- **Automated Testing**: Includes test cases for different operations, ensuring the reliability of data processing and querying functionalities.

## Getting Started

### Prerequisites

- Rust and Cargo installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).
- SQLite3 for database interactions (if needed for external verification).

### Installation

1. Clone this repository:

```bash
   git clone https://github.com/jessc0202/Sizhe_Chen_Individual_Project_2.git
   cd Sizhe_Chen_Individual_Project_2
```
### Preparation and Dependency Installation: 
1. open codespaces 
2. wait for codespaces to be built 
3. build: `cargo build` for dependencies installation
4. extract: `cargo run extract`
5. transform and load: `cargo run transform_load`
6. query sample: you can use `make create`, `make read`, `make update`, or `make delete` to see sample CRUD Operations
7. query your own: `cargo run query <insert own query here>`
8. You can find my successful CRUD operations [here](https://github.com/jessc0202/Sizhe_Chen_Individual_Project_2/blob/main/query_log.md)

### Check Format and Test Errors: 
1. Format code `make format`
2. Lint code `make lint`
3. Test coce `make test`

### Optimized Rust Binary
1. You can find and download the uploaded artifact by going to `actions` and clicking on the latest workflow run

### Use of Language Model (LLM)
In this project, I leveraged an LLM for:
1. **Generating Boilerplate Code:** Assisting with structured templates for core functions like `extract`, `transform_load`, and `query`.
2. **Debugging:** Suggesting fixes for errors and enhancing development speed.
3. **Documentation:** Helping craft the README.md file for clarity and thoroughness.

## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* https://github.com/nogibjj/rust-data-engineering
* https://docs.rs/sqlite/latest/sqlite/
* https://github.com/fivethirtyeight/data
