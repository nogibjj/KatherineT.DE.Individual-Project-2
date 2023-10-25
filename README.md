# Data Engineering Individual Project 2
## Purpose
This project Rust CLI Binary with SQLite aims to use Rust Script to interact with SQL database and demostrate CRUD operations. I created three functions that achieve extract data from a URL, transform and load the data into SQLite Database and query from the dataset. GitHub Copilot was used during the developing process. GitHub Actions was used to set up the environment, test, format and lint. Dataset used in this project is from FiveThirtyEight's public dataset, called Births. It contains births data from 2000-2014. 

## Steps 
1. Fork the repository at https://github.com/nogibjj/rust-data-engineering
2. Choose a command-line tool projects. (`calc-cli-with-tests` chosen here)
3. Create a new command line argument and create a test
4. Changing output format
5. Run `cargo build` and `cargo run` to make changes and test

## Tool Guidance
1. Install Rust and Cargo first
2. Check if `cargo.toml` is under the project folder. Also, make sure the working directory is the project folder. If not, run
```cd /path/to/your/directory```
3. Run
``` cargo build```
``` cargo run```.
It will get you something like this: 
![Screen Shot 2023-10-22 at 10 56 33 PM](https://github.com/nogibjj/KatherineT.DE.Mini-Project-7/assets/143833511/83c1b571-117c-4f3d-a022-930ec4f34e02)


5. My tool can return random fruits of Portugal or check if a fruit is native in Portugal.
   - To return random fruits, run
```cargo run -- --count 5```
This command will execute the program and request it to generate and print five random fruits. The --count flag with the value 5 specifies the number of fruits to generate.
   - To check if the fruit is native in Portugal, run
```cargo run -- -f <fruit_name>```
For example, you can run
```cargo run -- -f apple```
to check if apple is a native fruit. 

## Check format and test
Use `make format` and `make lint` to format and lint the code.
![Screen Shot 2023-10-22 at 11 00 06 PM](https://github.com/nogibjj/KatherineT.DE.Mini-Project-7/assets/143833511/4e4fa923-274d-4c97-8797-232137028465)

Use `make test` to test the code

![Screen Shot 2023-10-22 at 11 00 24 PM](https://github.com/nogibjj/KatherineT.DE.Mini-Project-7/assets/143833511/61d9684f-ba7a-4dd4-afc9-cf7e9ae735d1)
