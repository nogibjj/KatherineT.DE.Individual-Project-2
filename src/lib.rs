use rusqlite::{params, Connection, Error, Row};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

const LOG_FILE: &str = "query_log.md";
fn log_query(query: &str, log_file: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(log_file) {
        if let Err(err) = writeln!(file, "```sql\n{}\n```\n", query) {
            eprintln!("Error writing to log file: {:?}", err);
        }
    } else {
        eprintln!("Error opening log file for writing.");
    }
}

#[derive(Debug)]
pub struct Births {
    year: i32,
    month: i32,
    date_of_month: i32,
    day_of_week: i32,
    births: i32,
}

// Function to extract data from a URL and save to a file
pub fn extract(url: &str, file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let mut file = File::create(file_path)?;
    file.write_all(&content)?;
    Ok(file_path.to_string())
}

// Function to transform and load CSV data into an SQLite database
pub fn transform_load(dataset: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open an SQLite database connection
    let conn = Connection::open("BirthsDB.db")?;

    // Create a table to store Births data
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Births (
             year INTEGER,
             month INTEGER,
             date_of_month INTEGER,
             day_of_week INTEGER,
             births INTEGER
         )",
        params![],
    )?;

    // Open the CSV file and read its content
    let file = std::fs::File::open(dataset)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        // Parse the CSV fields into a Births struct
        let births = Births {
            year: record[0].parse()?,
            month: record[1].parse()?,
            date_of_month: record[2].parse()?,
            day_of_week: record[3].parse()?,
            births: record[4].parse()?,
        };

        // Insert the Births data into the SQLite table
        conn.execute(
            "INSERT INTO Births (year, month, date_of_month, day_of_week, births)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                births.year,
                births.month,
                births.date_of_month,
                births.day_of_week,
                births.births
            ],
        )?;
    }

    Ok(())
}

fn create_births_from_row(row: &Row) -> Result<Births, Error> {
    Ok(Births {
        year: row.get(0)?,
        month: row.get(1)?,
        date_of_month: row.get(2)?,
        day_of_week: row.get(3)?,
        births: row.get(4)?,
    })
}

// Function to execute a SQL query and return results as Births
pub fn query(query_string: &str) -> Result<Vec<Births>, rusqlite::Error> {
    let conn = Connection::open("BirthsDB.db")?;
    let mut stmt = conn.prepare(query_string)?;

    if query_string
        .trim_start()
        .to_uppercase()
        .starts_with("SELECT")
    {
        let births_iter = stmt.query_map([], |row| create_births_from_row(row))?;

        let mut result = Vec::new();
        for birth in births_iter {
            match birth {
                Ok(b) => {
                    result.push(b);
                }
                Err(e) => eprintln!("Error in row: {:?}", e),
            }
        }

        Ok(result)
    } else {
        conn.execute_batch(query_string)?;
        Ok(Vec::new()) // Return an empty vector for non-SELECT queries
    }
}
