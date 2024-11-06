use pest::Parser;
use pest_derive::Parser;
use serde::Serialize;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "proto.pest"]
pub struct ProtoParser;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Syntax error: {0}")]
    SyntaxError(String),
    #[error("Parse error: {0}")]
    ParseError(#[from] pest::error::Error<Rule>),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Serialize)]
pub struct Proto {
    syntax: String,
    package: Option<String>,
    imports: Vec<String>,
    messages: Vec<Message>,
    enums: Vec<EnumDef>,
    services: Vec<Service>,
}

#[derive(Debug, Serialize)]
pub struct Message {
    name: String,
    fields: Vec<Field>,
    nested_messages: Vec<Message>,
    nested_enums: Vec<EnumDef>,
}

#[derive(Debug, Serialize)]
pub struct Field {
    name: String,
    type_name: String,
    tag: i32,
    repeated: bool,
}

#[derive(Debug, Serialize)]
pub struct EnumDef {
    name: String,
    values: Vec<EnumValue>,
}

#[derive(Debug, Serialize)]
pub struct EnumValue {
    name: String,
    number: i32,
}

#[derive(Debug, Serialize)]
pub struct Service {
    name: String,
    methods: Vec<Method>,
}

#[derive(Debug, Serialize)]
pub struct Method {
    name: String,
    input_type: String,
    output_type: String,
}

impl Proto {
    pub fn parse_file(path: &str) -> Result<String, ParserError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    pub fn parse(input: &str) -> Result<String, ParserError> {
        // Parse the input using pest
        let pairs = ProtoParser::parse(Rule::proto_file, input)?;

        // Create a new Proto structure
        let mut proto = Proto {
            syntax: "proto3".to_string(), // default
            package: None,
            imports: Vec::new(),
            messages: Vec::new(),
            enums: Vec::new(),
            services: Vec::new(),
        };

        // Process all pairs
        for pair in pairs {
            match pair.as_rule() {
                Rule::syntax => {
                    proto.syntax = pair.into_inner()
                        .next()
                        .unwrap()
                        .as_str()
                        .trim_matches('"')
                        .to_string();
                }
                Rule::package => {
                    proto.package = Some(pair.into_inner()
                        .next()
                        .unwrap()
                        .as_str()
                        .to_string());
                }
                Rule::import => {
                    proto.imports.push(pair.into_inner()
                        .next()
                        .unwrap()
                        .as_str()
                        .trim_matches('"')
                        .to_string());
                }
                Rule::message_def => {
                    proto.messages.push(Self::parse_message(pair)?);
                }
                Rule::enum_def => {
                    proto.enums.push(Self::parse_enum(pair)?);
                }
                Rule::service_def => {
                    proto.services.push(Self::parse_service(pair)?);
                }
                _ => {}
            }
        }

        // Convert to JSON
        let json = serde_json::to_string_pretty(&proto)?;
        Ok(json)
    }

    fn parse_message(pair: pest::iterators::Pair<Rule>) -> Result<Message, ParserError> {
        let mut message = Message {
            name: String::new(),
            fields: Vec::new(),
            nested_messages: Vec::new(),
            nested_enums: Vec::new(),
        };

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ident => {
                    message.name = inner_pair.as_str().to_string();
                }
                Rule::field => {
                    message.fields.push(Self::parse_field(inner_pair)?);
                }
                Rule::message_def => {
                    message.nested_messages.push(Self::parse_message(inner_pair)?);
                }
                Rule::enum_def => {
                    message.nested_enums.push(Self::parse_enum(inner_pair)?);
                }
                _ => {}
            }
        }

        Ok(message)
    }

    fn parse_field(pair: pest::iterators::Pair<Rule>) -> Result<Field, ParserError> {
        let mut field = Field {
            name: String::new(),
            type_name: String::new(),
            tag: 0,
            repeated: false,
        };

        let mut inner_pairs = pair.into_inner();

        // Check for field rule (optional, repeated, required)
        if let Some(rule_pair) = inner_pairs.next() {
            if rule_pair.as_rule() == Rule::field_rule {
                field.repeated = rule_pair.as_str() == "repeated";
                // Get the next pair for type
                if let Some(type_pair) = inner_pairs.next() {
                    field.type_name = type_pair.as_str().to_string();
                }
            } else {
                field.type_name = rule_pair.as_str().to_string();
            }
        }

        // Get name and tag
        if let Some(name_pair) = inner_pairs.next() {
            field.name = name_pair.as_str().to_string();
        }
        if let Some(tag_pair) = inner_pairs.next() {
            field.tag = tag_pair.as_str().parse().unwrap_or(0);
        }

        Ok(field)
    }

    fn parse_enum(pair: pest::iterators::Pair<Rule>) -> Result<EnumDef, ParserError> {
        let mut enum_def = EnumDef {
            name: String::new(),
            values: Vec::new(),
        };

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ident => {
                    enum_def.name = inner_pair.as_str().to_string();
                }
                Rule::enum_value => {
                    let mut value = EnumValue {
                        name: String::new(),
                        number: 0,
                    };
                    let mut value_pairs = inner_pair.into_inner();
                    if let Some(name_pair) = value_pairs.next() {
                        value.name = name_pair.as_str().to_string();
                    }
                    if let Some(number_pair) = value_pairs.next() {
                        value.number = number_pair.as_str().parse().unwrap_or(0);
                    }
                    enum_def.values.push(value);
                }
                _ => {}
            }
        }

        Ok(enum_def)
    }

    fn parse_service(pair: pest::iterators::Pair<Rule>) -> Result<Service, ParserError> {
        let mut service = Service {
            name: String::new(),
            methods: Vec::new(),
        };

        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::ident => {
                    service.name = inner_pair.as_str().to_string();
                }
                Rule::rpc_def => {
                    let mut method = Method {
                        name: String::new(),
                        input_type: String::new(),
                        output_type: String::new(),
                    };

                    let mut rpc_pairs = inner_pair.into_inner();
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
                _ => {}
            }
        }

        Ok(service)
    }
}