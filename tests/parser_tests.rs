use proto_file_parser::{Proto, ParserError};

#[test]
fn test_parse_simple_message() -> Result<(), ParserError> {
    let input = r#"
        syntax = "proto3";

        message Test {
            string name = 1;
            int32 age = 2;
            repeated string hobbies = 3;
        }
    "#;

    let json = Proto::parse(input)?;
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(value["syntax"], "proto3");
    assert_eq!(value["messages"][0]["name"], "Test");
    assert_eq!(value["messages"][0]["fields"][0]["name"], "name");
    assert_eq!(value["messages"][0]["fields"][0]["type_name"], "string");
    assert_eq!(value["messages"][0]["fields"][0]["tag"], 1);
    assert_eq!(value["messages"][0]["fields"][1]["name"], "age");
    assert_eq!(value["messages"][0]["fields"][1]["type_name"], "int32");
    assert_eq!(value["messages"][0]["fields"][1]["tag"], 2);
    assert_eq!(value["messages"][0]["fields"][2]["name"], "hobbies");
    assert_eq!(value["messages"][0]["fields"][2]["type_name"], "string");
    assert_eq!(value["messages"][0]["fields"][2]["tag"], 3);
    assert_eq!(value["messages"][0]["fields"][2]["repeated"], true);

    Ok(())
}

#[test]
fn test_parse_nested_message() -> Result<(), ParserError> {
    let input = r#"
        syntax = "proto3";

        message Outer {
            string name = 1;
            message Inner {
                int32 value = 1;
            }
            Inner inner = 2;
        }
    "#;

    let json = Proto::parse(input)?;
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(value["syntax"], "proto3");
    assert_eq!(value["messages"][0]["name"], "Outer");
    assert_eq!(value["messages"][0]["nested_messages"][0]["name"], "Inner");

    Ok(())
}

#[test]
fn test_parse_enum() -> Result<(), ParserError> {
    let input = r#"
        syntax = "proto3";

        enum Color {
            UNKNOWN = 0;
            RED = 1;
            GREEN = 2;
            BLUE = 3;
        }
    "#;

    let json = Proto::parse(input)?;
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(value["syntax"], "proto3");
    assert_eq!(value["enums"][0]["name"], "Color");
    assert_eq!(value["enums"][0]["values"][0]["name"], "UNKNOWN");
    assert_eq!(value["enums"][0]["values"][0]["number"], 0);

    Ok(())
}

#[test]
fn test_parse_service() -> Result<(), ParserError> {
    let input = r#"
        syntax = "proto3";

        service Greeter {
            rpc SayHello (HelloRequest) returns (HelloResponse);
        }
    "#;

    let json = Proto::parse(input)?;
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();

    assert_eq!(value["syntax"], "proto3");
    assert_eq!(value["services"][0]["name"], "Greeter");
    assert_eq!(value["services"][0]["methods"][0]["name"], "SayHello");
    assert_eq!(value["services"][0]["methods"][0]["input_type"], "HelloRequest");
    assert_eq!(value["services"][0]["methods"][0]["output_type"], "HelloResponse");

    Ok(())
}