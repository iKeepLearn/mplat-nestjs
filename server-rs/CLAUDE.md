## Architecture

The project is organized as a Rust workspace with the following crates:

- **server/** - Main binary entry point and server setup
- **routes/** - HTTP route definitions and handlers
- **service/** - Business logic layer
- **database/** - Database access layer (repositories, transactions, migrations)
- **structs/** - Data transfer objects (DTOs) and SQL models
- **config/** - Application configuration loading
- **error/** - Error types and handling
- **utils/** - Utility functions (date/time, serde helpers,etc.)
- **actix-extensible-rate-limit/** - Custom rate limiting middleware for Actix-web