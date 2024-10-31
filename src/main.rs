use std::env;
use std::process;

// Import the functions from `lib.rs` using the crate name
use sizhe_chen_individual_project_2::{extract, transform_load, query};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [action]", args[0]);
        process::exit(1);
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            // Set URL and file paths for extraction
            let url = "https://raw.githubusercontent.com/fivethirtyeight/data/refs/heads/master/candy-power-ranking/candy-data.csv";
            let file_path = "data/serve_times.csv";
            let directory = "data";

            if let Err(err) = extract(url, file_path, directory) {
                eprintln!("Error extracting data: {:?}", err);
                process::exit(1);
            } else {
                println!("Extraction successful!");
            }
        }
        "transform_load" => {
            // Load data from a specified CSV file into the database
            if args.len() < 3 {
                eprintln!("Usage: {} transform_load [CSV file path]", args[0]);
                process::exit(1);
            }
            let csv_path = &args[2];
            match transform_load(csv_path) {
                Ok(conn) => {
                    println!("Data loaded successfully!");
                    drop(conn); // Close the connection after loading
                }
                Err(err) => {
                    eprintln!("Error loading data: {:?}", err);
                    process::exit(1);
                }
            }
        }
        "query" => {
            // Execute a SQL query on the database
            if args.len() < 3 {
                eprintln!("Usage: {} query [SQL query]", args[0]);
                process::exit(1);
            }
            let sql_query = &args[2];
            let csv_path = "data/serve_times.csv"; // Default CSV file path for loading data

            match transform_load(csv_path) {
                Ok(conn) => {
                    if let Err(err) = query(&conn, sql_query) {
                        eprintln!("Error executing query: {:?}", err);
                        process::exit(1);
                    } else {
                        println!("Query executed successfully!");
                    }
                }
                Err(err) => {
                    eprintln!("Error loading data: {:?}", err);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
            process::exit(1);
        }
    }
}
