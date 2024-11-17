# proto-file-parser

A Rust library and CLI tool for parsing Protocol Buffer Definition (.proto) files into JSON format.

https://crates.io/crates/proto-file-parser  
https://docs.rs/proto-file-parser/latest/proto_file_parser/

## Technical Description

The parser processes .proto files through the following stages:

1. **Lexical Analysis**
    - Tokenizes the input into Protocol Buffer language elements
    - Handles comments, whitespace, and line continuations
    - Recognizes keywords, identifiers, numbers, and special characters

2. **Syntax Parsing**
    - Parses the token stream into an Abstract Syntax Tree (AST)
    - Validates syntax according to Protocol Buffer Language Specification
    - Handles nested message definitions and imports

3. **Semantic Analysis**
    - Validates field numbers and types
    - Ensures enum values are unique
    - Verifies package names and imports

4. **JSON Generation**
    - Converts the AST into a structured JSON format
    - Preserves all relevant information from the .proto file
    - Maintains nested structure relationships

## Grammar Rules

```ebnf
proto          = syntax_spec package_spec? import_spec* 
                 (message_def | enum_def | service_def)*

syntax_spec    = "syntax" "=" quote ("proto2" | "proto3") quote ";"

package_spec   = "package" full_ident ";"

import_spec    = "import" quote import_path quote ";"

message_def    = "message" ident "{" field_def* "}"

field_def      = type_name ident "=" number ";"

enum_def       = "enum" ident "{" enum_field* "}"

service_def    = "service" ident "{" rpc_def* "}"

rpc_def        = "rpc" ident "(" message_type ")" 
                 "returns" "(" message_type ")" ";"
```

## Usage

### CLI

```bash
# Parse a .proto file and output JSON
proto-file-parser parse input.proto

# Display help information
proto-file-parser help

# Show version and credits
proto-file-parser version
```

### Library

```rust
use proto_parser::Parser;

let parser = Parser::new();
let json = parser.parse_file("input.proto")?;
println!("{}", json);
```

### Building and Testing

```bash
# Format code
make format

# Run tests
make test

# Run linter
make lint

# Run all checks before committing
make pre-commit
```