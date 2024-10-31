use std::env;
use std::process;

// Import the functions from `lib.rs` using the crate name
use sizhe_chen_individual_project_2::{transform_load, query};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [action]", args[0]);
        process::exit(1);
    }

    let action = &args[1];
    match action.as_str() {
        "transform_load" => {
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
            if args.len() < 4 {
                eprintln!("Usage: {} query [CSV file path] [SQL query]", args[0]);
                process::exit(1);
            }
            let csv_path = &args[2];
            let sql_query = &args[3];
            match transform_load(csv_path) {
                Ok(conn) => {
                    if let Err(err) = query(&conn, sql_query) {
                        eprintln!("Error executing query: {:?}", err);
                        process::exit(1);
                    }
                }
                Err(err) => {
                    eprintln!("Error loading data: {:?}", err);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Invalid action. Use 'transform_load' or 'query'.");
            process::exit(1);
        }
    }
}
