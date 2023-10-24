use rust_invidivual_project::{extract, query, transform_load};
use std::env; // Import the log_query function

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];
    match action.as_str() {
        "extract" => {
            let result = extract(
                "https://github.com/fivethirtyeight/data/blob/master/births/US_births_2000-2014_SSA.csv?raw=true",
                "births.csv",
            );
            match result {
                Ok(path) => println!("Data extraction completed successfully. Saved to {}", path),
                Err(e) => eprintln!("Error during extraction: {:?}", e),
            }
        }
        "transform_load" => match transform_load("births.csv") {
            Ok(()) => println!("Data transformation and loading completed successfully."),
            Err(e) => eprintln!("Error during loading: {:?}", e),
        },
        "query" => {
            if args.len() < 3 {
                println!("Please provide a SQL query string");
                return;
            }
            let query_string = &args[2];
            query(query_string).unwrap();
        }
        _ => {
            println!("Invalid action. Use 'extract', 'transform_load', or 'query'.");
        }
    }
}
