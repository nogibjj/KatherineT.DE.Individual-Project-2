use rust_invidivual_project::{extract, query, transform_load};

#[cfg(test)]
mod tests {
    // Import the functions you want to test
    use super::{extract, query, transform_load};

    // Write test functions
    #[test]
    fn test_extract() {
        // Test the extract function
        let result = extract("https://github.com/fivethirtyeight/data/blob/master/births/US_births_2000-2014_SSA.csv?raw=true", "test.csv");
        assert!(result.is_ok()); // Check if the result is Ok
    }

    #[test]
    fn test_transform_load() {
        // Test the transform_load function
        let result = transform_load("test.csv");
        match result {
            Ok(()) => println!("Transform and load succeeded."),
            Err(e) => {
                eprintln!("Error during loading: {:?}", e);
                panic!("Transform and load failed.");
            }
        }
    }

    #[test]
    fn test_query() {
        // Test the query function
        let result = query("SELECT * FROM Births");
        match result {
            Ok(_) => println!("Query succeeded."),
            Err(e) => {
                eprintln!("Error during query: {:?}", e);
                panic!("Query failed.");
            }
        }
    }
}
