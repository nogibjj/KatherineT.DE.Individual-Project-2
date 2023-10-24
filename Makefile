install:
	@echo "Checking if Rust is installed..."
	@which rustc &> /dev/null || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	@echo "Installing project dependencies..."
	@cargo build


# Display Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version      # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test --quiet

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Extract data
extract: 
	cargo run extract

# Transform and Load data
transform_load:
	cargo run transform_load

# Query the top 10 rows from the BirthsDB table where year is 2000
query:
	cargo run query

create-table:
	cargo run query "CREATE TABLE IF NOT EXISTS BirthsDB (year INTEGER, month INTEGER, date_of_month INTEGER, day_of_week INTEGER, births INTEGER);"

create:
	cargo run query "INSERT INTO BirthsDB (year,month,date_of_month,day_of_week,births) VALUES (2000,10,2,3,10000);"

# Read specific entries using the year
read:
	cargo run query "SELECT * FROM BirthsDB WHERE year = 2001;"

# Update several entries in the BirthsDB table
update:
	cargo run query "UPDATE BirthsDB SET births=7869 WHERE year = 2003;"

# Delete entries from the BirthsDB table
delete:
	cargo run query "DELETE FROM BirthsDB WHERE year = 2003;"

# Run all formatting, linting, and testing tasks
all: format lint test run

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local usetest.rsr.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi
