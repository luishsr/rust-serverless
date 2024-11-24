
# Rust Serverless Platform

This project demonstrates how to build a serverless platform in Rust powered by WebAssembly. It allows dynamic function registration and invocation using WebAssembly modules.

## Features
- **Dynamic Function Registration**: Register WebAssembly modules via a simple HTTP API.
- **Dynamic Function Invocation**: Execute registered functions with runtime-supplied inputs.

## Tutorial
A comprehensive tutorial on building this platform is available [here](https://luissoares.dev/building-a-rust-serverless-platform/).

## Setup
1. Clone the repository:
   ```bash
   git clone https://github.com/luishsr/rust-serverless.git
   cd rust-serverless
   ```
2. Install dependencies and run the server:
   ```bash
   cargo run
   ```

## Example Usage
### Register a Function
```bash
curl -X POST http://127.0.0.1:3030/register     -H "Content-Type: application/json"     -d '{
        "name": "add",
        "code": "(module (func (export \"add\") (param i32 i32) (result i32) local.get 0 local.get 1 i32.add))"
    }'
```

### Invoke the Function
```bash
curl -X POST http://127.0.0.1:3030/invoke     -H "Content-Type: application/json"     -d '{
        "name": "add",
        "input": [3, 7]
    }'
```

## Tests
Run the tests to verify the functionality:
```bash
cargo test
```

## License
This project is licensed under the MIT License.

## Author
Developed by [Luís Soares](https://github.com/luishsr).

## Complete Source Code
The source code for this project is available on GitHub:  
[https://github.com/luishsr/rust-serverless](https://github.com/luishsr/rust-serverless)
