use pest::Parser;
use pest_derive::Parser;
use serde::Serialize;
use thiserror::Error;

/// Parser implementation using pest grammar rules.
/// This struct is used to parse Protocol Buffer files according to the grammar defined in proto.pest.
#[derive(Parser)]
#[grammar = "proto.pest"]
pub struct ProtoParser;

/// Represents all possible errors that can occur during parsing and processing of proto files.
#[derive(Error, Debug)]
pub enum ParserError {
    /// Indicates an error in the proto syntax structure
    #[error("Syntax error: {0}")]
    SyntaxError(String),

    /// Indicates an error during the parsing process
    #[error("Parse error: {0}")]
    ParseError(#[from] pest::error::Error<Rule>),

    /// Indicates an error during file operations
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Indicates an error during JSON serialization
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Main structure representing a complete Protocol Buffer file.
/// Contains all the elements that can be defined in a proto file.
#[derive(Debug, Serialize)]
pub struct Proto {
    /// The syntax version specified in the proto file (e.g., "proto3")
    syntax: String,
    /// Optional package name that scopes the proto definitions
    package: Option<String>,
    /// List of other proto files that are imported
    imports: Vec<String>,
    /// List of message type definitions
    messages: Vec<Message>,
    /// List of enum type definitions
    enums: Vec<EnumDef>,
    /// List of service definitions
    services: Vec<Service>,
}

/// Represents a message definition in the proto file.
/// Messages are user-defined composite types.
#[derive(Debug, Serialize)]
pub struct Message {
    /// Name of the message type
    name: String,
    /// List of fields contained in the message
    fields: Vec<Field>,
    /// List of message types defined within this message
    nested_messages: Vec<Message>,
    /// List of enum types defined within this message
    nested_enums: Vec<EnumDef>,
}

/// Represents a field within a message.
/// Fields are the basic components of a message.
#[derive(Debug, Serialize)]
pub struct Field {
    /// Name of the field
    name: String,
    /// Type of the field (can be primitive type or another message type)
    type_name: String,
    /// Unique numerical tag that identifies the field in the message
    tag: i32,
    /// Indicates if the field is a repeated field (array/list)
    repeated: bool,
}

/// Represents an enumeration definition.
/// Enums are a type that can have one of a predefined set of values.
#[derive(Debug, Serialize)]
pub struct EnumDef {
    /// Name of the enum type
    name: String,
    /// List of possible values for this enum
    values: Vec<EnumValue>,
}

/// Represents a single value in an enum definition.
#[derive(Debug, Serialize)]
pub struct EnumValue {
    /// Name of the enum value (should be UPPERCASE_WITH_UNDERSCORES by convention)
    name: String,
    /// Integer value associated with this enum value
    number: i32,
}

/// Represents a service definition.
/// Services define methods that can be called remotely.
#[derive(Debug, Serialize)]
pub struct Service {
    /// Name of the service
    name: String,
    /// List of methods provided by this service
    methods: Vec<Method>,
}

/// Represents an RPC method in a service definition.
#[derive(Debug, Serialize)]
pub struct Method {
    /// Name of the method
    name: String,
    /// Type of the input message
    input_type: String,
    /// Type of the output message
    output_type: String,
}

impl Proto {
    /// Parses a proto file from the filesystem and returns its JSON representation.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the proto file to be parsed
    ///
    /// # Returns
    ///
    /// A Result containing either the JSON string representation of the parsed proto file
    /// or a ParserError if any error occurs during parsing or processing.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use proto_file_parser::Proto;
    ///
    /// let json = Proto::parse_file("example.proto").unwrap();
    /// println!("{}", json);
    /// ```
    pub fn parse_file(path: &str) -> Result<String, ParserError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// Parses a proto definition from a string and returns its JSON representation.
    ///
    /// # Arguments
    ///
    /// * `input` - String containing the proto definition to be parsed
    ///
    /// # Returns
    ///
    /// A Result containing either the JSON string representation of the parsed proto definition
    /// or a ParserError if any error occurs during parsing or processing.
    ///
    /// # Examples
    ///
    /// ```
    /// use proto_file_parser::Proto;
    ///
    /// let input = r#"
    ///     syntax = "proto3";
    ///     message Test {
    ///         string name = 1;
    ///     }
    /// "#;
    ///
    /// let json = Proto::parse(input).unwrap();
    /// println!("{}", json);
    /// ```
    pub fn parse(input: &str) -> Result<String, ParserError> {
        let pairs = ProtoParser::parse(Rule::proto_file, input)?;

        let mut proto = Proto {
            syntax: "proto3".to_string(),
            package: None,
            imports: Vec::new(),
            messages: Vec::new(),
            enums: Vec::new(),
            services: Vec::new(),
        };

        for pair in pairs {
            match pair.as_rule() {
                Rule::proto_file => {
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::syntax => {
                                proto.syntax = inner_pair
                                    .into_inner()
                                    .next()
                                    .unwrap()
                                    .as_str()
                                    .trim_matches('"')
                                    .to_string();
                            }
                            Rule::package => {
                                proto.package = Some(
                                    inner_pair
                                        .into_inner()
                                        .next()
                                        .unwrap()
                                        .as_str()
                                        .to_string(),
                                );
                            }
                            Rule::import => {
                                proto.imports.push(
                                    inner_pair
                                        .into_inner()
                                        .next()
                                        .unwrap()
                                        .as_str()
                                        .trim_matches('"')
                                        .to_string(),
                                );
                            }
                            Rule::message_def => {
                                proto.messages.push(Self::parse_message(inner_pair)?);
                            }
                            Rule::enum_def => {
                                proto.enums.push(Self::parse_enum(inner_pair)?);
                            }
                            Rule::service_def => {
                                proto.services.push(Self::parse_service(inner_pair)?);
                            }
                            Rule::EOI => {}
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        let json = serde_json::to_string_pretty(&proto)?;
        Ok(json)
    }

    /// Parses a message definition from a pest Pair.
    fn parse_message(pair: pest::iterators::Pair<Rule>) -> Result<Message, ParserError> {
        let mut message = Message {
            name: String::new(),
            fields: Vec::new(),
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
        };

        let mut pairs = pair.into_inner();

        if let Some(name_pair) = pairs.next() {
            if name_pair.as_rule() == Rule::ident {
                message.name = name_pair.as_str().to_string();
            }
        }

        for pair in pairs {
            match pair.as_rule() {
                Rule::field => {
                    message.fields.push(Self::parse_field(pair)?);
                }
                Rule::message_def => {
                    message.nested_messages.push(Self::parse_message(pair)?);
                }
                Rule::enum_def => {
                    message.nested_enums.push(Self::parse_enum(pair)?);
                }
                _ => {}
            }
        }

        Ok(message)
    }

    /// Parses a field definition from a pest Pair.
    fn parse_field(pair: pest::iterators::Pair<Rule>) -> Result<Field, ParserError> {
        let mut field = Field {
            name: String::new(),
            type_name: String::new(),
            tag: 0,
            repeated: false,
        };

        let mut pairs = pair.into_inner().peekable();

        if let Some(first_pair) = pairs.peek() {
            if first_pair.as_rule() == Rule::field_rule {
                field.repeated = first_pair.as_str() == "repeated";
                pairs.next();
            }
        }

        if let Some(type_pair) = pairs.next() {
            field.type_name = type_pair.as_str().to_string();
        }

        if let Some(name_pair) = pairs.next() {
            field.name = name_pair.as_str().to_string();
        }

        if let Some(tag_pair) = pairs.next() {
            field.tag = tag_pair.as_str().parse().unwrap_or(0);
        }

        Ok(field)
    }

    /// Parses an enum definition from a pest Pair.
    fn parse_enum(pair: pest::iterators::Pair<Rule>) -> Result<EnumDef, ParserError> {
        let mut enum_def = EnumDef {
            name: String::new(),
            values: Vec::new(),
        };

        let mut pairs = pair.into_inner();

        if let Some(name_pair) = pairs.next() {
            if name_pair.as_rule() == Rule::ident {
                enum_def.name = name_pair.as_str().to_string();
            }
        }

        for pair in pairs {
            if pair.as_rule() == Rule::enum_value {
                let mut value_pairs = pair.into_inner();
                let mut enum_value = EnumValue {
                    name: String::new(),
                    number: 0,
                };

                if let Some(name_pair) = value_pairs.next() {
                    enum_value.name = name_pair.as_str().to_string();
                }
                if let Some(number_pair) = value_pairs.next() {
                    enum_value.number = number_pair.as_str().parse().unwrap_or(0);
                }

                enum_def.values.push(enum_value);
            }
        }

        Ok(enum_def)
    }

    /// Parses a service definition from a pest Pair.
    fn parse_service(pair: pest::iterators::Pair<Rule>) -> Result<Service, ParserError> {
        let mut service = Service {
            name: String::new(),
            methods: Vec::new(),
        };

        let mut pairs = pair.into_inner();

        if let Some(name_pair) = pairs.next() {
            if name_pair.as_rule() == Rule::ident {
                service.name = name_pair.as_str().to_string();
            }
        }

        for pair in pairs {
            if pair.as_rule() == Rule::rpc_def {
                let mut method = Method {
                    name: String::new(),
                    input_type: String::new(),
                    output_type: String::new(),
                };

                let mut rpc_pairs = pair.into_inner();

                if let Some(name_pair) = rpc_pairs.next() {
                    method.name = name_pair.as_str().to_string();
                }
                if let Some(input_pair) = rpc_pairs.next() {
                    method.input_type = input_pair.into_inner().next().unwrap().as_str().to_string();
                }
                if let Some(output_pair) = rpc_pairs.next() {
                    method.output_type = output_pair.into_inner().next().unwrap().as_str().to_string();
                }

                service.methods.push(method);
            }
        }

        Ok(service)
    }
}