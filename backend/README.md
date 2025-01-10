# Learning Management System (LMS) Server

A robust Learning Management System backend API built with Rust using MongoDB for data persistence.

## Project Structure

```
LMS-SERVER/
├── src/
│   ├── configs/      # Configuration management
│   ├── database/     # Database connection
│   ├── errors/       # Custom error definitions
│   ├── handlers/     # Request handlers
│   ├── jobs/         # Cron jobs to automate process
│   ├── middlewares/  # Custom middleware
│   ├── models/       # Data models
│   ├── repositories/ # Data access layer
│   ├── routes/       # API route definitions
│   ├── services/     # Business logic
│   ├── utils/        # Utility functions
│   ├── lib.rs        # Library entry point
│   └── main.rs       # Application entry point
├── tests/            # Integration and Unit tests
└── logs/             # Application logs
```

## Prerequisites

- Rust (latest stable version)
- MongoDB
- Redis (optional, for caching)

## Local Development Setup

### 1. Install Rust

#### Windows

1. Download Rust installer from [rustup.rs](https://rustup.rs/)
2. Run the downloaded `rustup-init.exe`
3. Choose option 1 for default installation
4. Restart terminal

#### Ubuntu/Debian

```bash
# Install build essentials
sudo apt update
sudo apt install build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH
source $HOME/.cargo/env
```

#### macOS

```bash
# Install Xcode Command Line Tools
xcode-select --install

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add to PATH
source $HOME/.cargo/env
```

#### Verify Installation

```bash
rustc --version
cargo --version
```

### 2. Install Required Components

```bash
rustup component add rustfmt
rustup component add clippy
```

### 3. Clone Repository

```bash
git clone -b develop https://github.com/lms-2025/lms-web.git
cd lms-web/backend
```

### 4. Environment Setup

#### Windows

```powershell
# Using PowerShell or Command Prompt
copy .env.example .env
```

#### Ubuntu/Debian & macOS

```bash
cp .env.example .env
```

Configure your `.env` file:

```env
DATABASE_URL=mongodb://localhost:27017
DATABASE_NAME=lms_db
PORT=8080
RUST_LOG=debug
JWT_SECRET=your_jwt_secret_key
EMAIL=email
EMAIL_PASSWORD=email password
REDIS_URL=redis_url
```

### 5. MongoDB Setup

```bash
# Start MongoDB service
sudo systemctl start mongod

# Verify MongoDB is running
mongo --eval 'db.runCommand({ connectionStatus: 1 })'
```

## Running the Application

### Development Mode

To automatically restart the server during development when code changes, install and use `cargo-watch`:

#### Install cargo-watch (only required once)

```markdown
cargo install cargo-watch
```

#### Run the server with automatic restart on changes

```markdown
cargo watch -w src -x 'run'
```

```markdown
cargo watch -i target -i logs -x 'run'
```

#### Run the server with no automatic restart on changes

```bash
cargo run
```

### Production Build

```bash
cargo build --release
./target/release/lms_server
```

## Development Commands

### Format Code

```bash
cargo fmt
```

### Run Linter

```bash
cargo clippy
```

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

## API Documentation

API documentation is available at `/docs` endpoint when running the server locally.

## Contributing

1. Create a new branch from `develop`
2. Make your changes
3. Run tests and ensure they pass
4. Run formatter and linter
5. Submit PR with description of changes

   ### Branch Naming

   ```
   feature/add-user-auth
   ```

   ### Commit Conventions

   Use conventional commits in the format: `type(scope): message`

   Types:

   - `feat`: New features (`feat(auth): add JWT authentication`)
   - `fix`: Bug fixes (`fix(db): handle MongoDB timeout`)
   - `docs`: Documentation (`docs(api): update endpoints`)
   - `test`: Adding/updating tests (`test(user): add validation tests`)
   - `refactor`: Code changes (`refactor(handlers): simplify error handling`)
   - `style`: Formatting (`style(global): apply rustfmt`)
   - `chore`: Maintenance (`chore(deps): update dependencies`)

   Examples:

   ```
   feat(courses): add course creation endpoint
   fix(auth): handle token expiration
   docs(readme): update installation steps
   test(api): add integration tests for auth
   refactor(models): restructure user schema
   ```

## Code Quality Standards

- All new code must have tests
- Maintain 80% or higher test coverage
- Run formatter and linter before commits
- Follow Rust standard naming conventions
- Document public APIs using rustdoc

## Common Issues & Solutions

### Database Connection Issues

- Verify MongoDB service is running
- Check MongoDB connection string in .env
- Ensure network connectivity
- Verify database permissions

### Build Errors

- Run `cargo clean` then rebuild
- Update Rust to latest stable
- Check for conflicting dependencies

## License

[MIT License](../LICENSE)
