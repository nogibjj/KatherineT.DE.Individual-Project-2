use rust_invidivual_project::{extract, transform_load, query}; // Import the log_query function

#[test]
fn test_extract_and_transform_load() {
    let url = "https://github.com/fivethirtyeight/data/blob/master/births/US_births_2000-2014_SSA.csv?raw=true";
    let file_path = "data/births.csv";

    // Extract
    extract(url, file_path).unwrap();
    assert!(std::fs::metadata(file_path).is_ok());

    // Transform and Load
    let result = transform_load(file_path);
    assert_eq!(result.unwrap(), "BirthsDB.db");
}

#[test]
fn test_query() {
    // Query the top 5 rows from the CarsDB table
    let result = query("SELECT * FROM BirthsDB WHERE year=2000 LIMIT 10;");

    // Check if the query was successful and returns 10 rows
    assert!(result.is_ok());
}