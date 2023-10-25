# Data Engineering Individual Project 2
## Purpose
This project Rust CLI Binary with SQLite aims to use Rust Script to interact with SQL database and demostrate CRUD operations. I created three functions that achieve extract data from a URL, transform and load the data into SQLite Database and query from the dataset. GitHub Copilot was used during the developing process. GitHub Actions was used to set up the environment, test, format and lint. Dataset used in this project is from FiveThirtyEight's public dataset, called Births. It contains births data from 2000-2014. 

## Steps 
1. Cargo init to add rust constructions
2. In `lib.rs`, create three functions that extract from a url and save as a csv, transform and load the csv file into a db file and query with SQL.
3. Create the `main.rs` that apply the functions in our dataset `births`.
4. Run `make format` and `make lint` to format and lint the code.
   
![Screen Shot 2023-10-24 at 7 41 12 PM](https://github.com/nogibjj/KatherineT.DE.Individual-Project-2/assets/143833511/0bdb5a8d-5db7-4631-af49-1f61c9eba338)

5. Create a test file to test three functions, and test by running `cargo test`.
My test result is shown here:

![Screen Shot 2023-10-24 at 7 40 49 PM](https://github.com/nogibjj/KatherineT.DE.Individual-Project-2/assets/143833511/78417c36-0bfe-42f0-8b4d-c0dae0f289c8)

## Tool Guidance
1. Install Rust and Cargo first
2. Check if `cargo.toml` is under the project folder. Also, make sure the working directory is the project folder. If not, run
```cd /path/to/your/directory```
3. Run ``` cargo build``` for dependencies installation
4. My tool can achieve several different functionalities.
   - To extract from a url and save as a csv, run
```cargo run extract```
   - To tranform and load the dataset
```cargo run transform_load```
   - To execute CRUD operations
 `make create`,`make read`,`make update`,`make delete`

## Copilot Utilization
**Code Completion**: Copilot provided code completion suggestions as I typed, helping me quickly finish lines of code. When I typed a method or variable name, it suggested the next part of the code for me.

**Refactoring Code:** Copilot helped me refactor my code by suggesting improvements. For example, it identified areas of code that can be optimized, suggest better variable names, and even suggest alternative solutions.

**Debugging**: Copilot suggested potential solutions to programming issues or bugs. It helped me understand error messages and provided hints on how to resolve them.

## GitHub Actions
The project uses GitHub Actions for continuous integration and continuous deployment (CI/CD). The workflow includes steps for:
- Format
- Lint
- Tests
- Build the Rust binary

## Demo Video
This is the link to my demo video:
https://youtu.be/uNu-MPrMfww
