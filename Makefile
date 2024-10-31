# Check Rust command-line utility versions
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version              # Rust compiler
	cargo --version              # Rust package manager
	rustfmt --version            # Rust code formatter
	rustup --version             # Rust toolchain manager
	clippy-driver --version || echo "Clippy not installed"  # Rust linter

# Format code using rustfmt
format:
	cargo fmt --quiet

# Run clippy for linting
lint:
	cargo clippy --quiet

# Run tests
test:
	cargo test

# Build and run the project
run:
	cargo run

# Build release version
release:
	cargo build --release

# Install Rust toolchain if needed
install:
	# Uncomment below to update rust toolchain
	# @echo "Updating rust toolchain"
	# rustup update stable
	# rustup default stable 

# Run all formatting, linting, and testing tasks
all: format lint test run

# Custom tasks for specific project actions
extract: 
	cargo run -- extract data/candy-data.csv

# Transform and Load data
transform_load:
	cargo run -- transform_load data/candy-data.csv

# Example: Create a database entry
create:
	cargo run -- query "INSERT INTO CandyData (competitorname, chocolate, fruity, caramel, peanutyalmondy, nougat, crispedricewafer, hard, bar, pluribus, sugarpercent, pricepercent, winpercent) VALUES ('Sample Candy', 1, 0, 1, 0, 0, 1, 0, 1, 0, 0.5, 0.3, 75.0);"

# Example: Read from the database
read:
	cargo run -- query "SELECT * FROM CandyData WHERE competitorname = 'Sample Candy';"

# Example: Update a database entry
update:
	cargo run -- query "UPDATE CandyData SET winpercent=80.0 WHERE competitorname='Sample Candy';"

# Example: Delete a database entry
delete:
	cargo run -- query "DELETE FROM CandyData WHERE competitorname='Sample Candy';"

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi
