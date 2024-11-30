# Project Title

## Setup

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository.
3. Run the following commands to run the server:

```sh
cargo build
cargo test
cargo run
```

4. (Optional) To use cargo commands:

```bash
cargo install cargo-commander
# Later: cargo cmd <command>
```

5. (Optional) Auto-restart server

```sh
cargo install cargo-watch
cargo watch -x run
```

## User Security

If implementing a user authentication system, remember to take care of:

- CORS: Cross site requests should not be allowed
- Rainbow attacks: Prohibit the user from setting insecure passwords
- Timing attacks: Make sure there is a delay after entering incorrect password X number of times
- Dynamic timing attacks: Add a random small (ms) timer before success login or failure and success/failure timings should be similar
