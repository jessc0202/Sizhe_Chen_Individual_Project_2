use rusqlite::{params, Connection};
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Load data from a CSV file into an SQLite database
pub fn transform_load(csv_path: &str) -> Result<Connection> {
    let conn = Connection::open_in_memory()?; // Create an in-memory SQLite database

    // Create the CandyData table
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
    
    Ok(conn)
}

// Execute a SQL query on the database and print the results
pub fn query(conn: &Connection, sql_query: &str) -> Result<()> {
    let mut stmt = conn.prepare(sql_query)?;
    let mut rows = stmt.query([])?;

    // Print each row in the query result
    while let Some(row) = rows.next()? {
        let competitorname: String = row.get(0)?;
        let winpercent: f64 = row.get(12)?;
        println!("Competitor: {}, Win Percent: {}", competitorname, winpercent);
    }
    
    Ok(())
}
