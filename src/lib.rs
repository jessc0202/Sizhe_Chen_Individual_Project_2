use anyhow::Result;
use reqwest::blocking::Client;
use rusqlite::{params, Connection};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

const LOG_FILE: &str = "query_log.md";

// Function to log queries in a markdown file
fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

// Function to download a file from a URL
pub fn extract(url: &str, file_path: &str, directory: &str) -> Result<()> {
    // Check if the directory exists; if not, create it
    if fs::metadata(directory).is_err() {
        fs::create_dir_all(directory)?;
    }

    // Create a blocking client to send the HTTP request
    let client = Client::new();
    let mut response = client.get(url).send()?;

    // Check if the response is successful
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to download file: HTTP {}",
            response.status()
        ));
    }

    // Create the file at the specified file_path
    let mut file = File::create(file_path)?;

    // Copy the response content into the file
    std::io::copy(&mut response, &mut file)?;

    println!("Extraction successful!");
    Ok(())
}

// Load data from a CSV file into an SQLite database
pub fn transform_load(csv_path: &str) -> Result<Connection> {
    let conn = Connection::open("CandyDataDB.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS CandyData (
            competitorname TEXT,
            chocolate INTEGER,
            fruity INTEGER,
            caramel INTEGER,
            peanutyalmondy INTEGER,
            nougat INTEGER,
            crispedricewafer INTEGER,
            hard INTEGER,
            bar INTEGER,
            pluribus INTEGER,
            sugarpercent REAL,
            pricepercent REAL,
            winpercent REAL
        )",
        [],
    )?;

    // Open the CSV file and read each line to insert into the database
    let file = File::open(csv_path)?;
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if index == 0 {
            continue; // Skip the header row
        }
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();
        conn.execute(
            "INSERT INTO CandyData (competitorname, chocolate, fruity, caramel, peanutyalmondy, nougat,
                                    crispedricewafer, hard, bar, pluribus, sugarpercent, pricepercent, winpercent)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                fields[0],
                fields[1].parse::<i32>().unwrap_or(0),
                fields[2].parse::<i32>().unwrap_or(0),
                fields[3].parse::<i32>().unwrap_or(0),
                fields[4].parse::<i32>().unwrap_or(0),
                fields[5].parse::<i32>().unwrap_or(0),
                fields[6].parse::<i32>().unwrap_or(0),
                fields[7].parse::<i32>().unwrap_or(0),
                fields[8].parse::<i32>().unwrap_or(0),
                fields[9].parse::<i32>().unwrap_or(0),
                fields[10].parse::<f64>().unwrap_or(0.0),
                fields[11].parse::<f64>().unwrap_or(0.0),
                fields[12].parse::<f64>().unwrap_or(0.0),
            ],
        )?;
    }

    println!("Data loaded into database successfully.");
    Ok(conn)
}

// Execute a SQL query on the database and print the results
pub fn query(conn: &Connection, sql_query: &str) -> Result<()> {
    if sql_query.trim().to_lowercase().starts_with("select") {
        let mut stmt = conn.prepare(sql_query)?;
        let mut rows = stmt.query([])?;

        // Print each row in the query result
        while let Some(row) = rows.next()? {
            let competitorname: String = row.get(0)?;
            let winpercent: f64 = row.get(12)?;
            println!(
                "Competitor: {}, Win Percent: {}",
                competitorname, winpercent
            );
        }
    } else {
        // For other operations (INSERT, UPDATE, DELETE)
        let _num_affected_rows = conn.execute_batch(sql_query)?;
    }
    log_query(sql_query, LOG_FILE);
    Ok(())
}
