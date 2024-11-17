#[cfg(test)]
mod tests {
    use proto_file_parser::{ParserError, Proto};
    // Testing WHITESPACE rule
    #[test]
    fn test_whitespace_rule() -> Result<(), ParserError> {
        let input = "syntax    =     \"proto3\";\n\t\rmessage Test {\n    string name = 1;\r\n}";
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["syntax"], "proto3");
        assert_eq!(value["messages"][0]["name"], "Test");
        Ok(())
    }

    // Testing COMMENT rule
    #[test]
    fn test_comment_rule() -> Result<(), ParserError> {
        let input = r#"
        // Single line comment
        syntax = "proto3"; // End of line comment
        /* Multi-line
           comment */
        message Test { // Comment after message
            // Comment before field
            string name = 1; /* Multi-line
                              field comment */
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["syntax"], "proto3");
        assert_eq!(value["messages"][0]["name"], "Test");
        Ok(())
    }

    // Testing ident rule
    #[test]
    fn test_ident_rule() -> Result<(), ParserError> {
        let input = r#"
    syntax = "proto3";
    // Testing various valid identifiers
    message ValidIdents {
        string normal = 1;
        string withUnderscore = 2;
        string with123Numbers = 3;
        string startWithLetter = 4;
        string camelCase = 5;
        string PascalCase = 6;
        string UPPERCASE = 7;
        string mixtureOfALLStyles123 = 8;
    }
"#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let fields = &value["messages"][0]["fields"];
        assert_eq!(fields[0]["name"], "normal");
        assert_eq!(fields[1]["name"], "withUnderscore");
        assert_eq!(fields[2]["name"], "with123Numbers");
        assert_eq!(fields[3]["name"], "startWithLetter");
        assert_eq!(fields[4]["name"], "camelCase");
        assert_eq!(fields[5]["name"], "PascalCase");
        assert_eq!(fields[6]["name"], "UPPERCASE");
        assert_eq!(fields[7]["name"], "mixtureOfALLStyles123");
        Ok(())
    }

    // Testing number rule
    #[test]
    fn test_number_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        message NumberTest {
            string field1 = 1;
            string field2 = 12;
            string field3 = 123;
            string field4 = 1234;
            string field5 = 12345;
            string field6 = 123456;
            string field7 = 536870911; // Max allowed
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let fields = &value["messages"][0]["fields"];
        assert_eq!(fields[0]["tag"], 1);
        assert_eq!(fields[1]["tag"], 12);
        assert_eq!(fields[2]["tag"], 123);
        assert_eq!(fields[3]["tag"], 1234);
        assert_eq!(fields[4]["tag"], 12345);
        assert_eq!(fields[5]["tag"], 123456);
        assert_eq!(fields[6]["tag"], 536870911);
        Ok(())
    }

    // Testing string_lit rule
    #[test]
    fn test_string_lit_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        import "simple/path.proto";
        import "path/with/multiple/segments.proto";
        import "path_with_underscore.proto";
        import "path.with.dots.proto";
        import "path-with-dashes.proto";
        import "UPPERCASE.proto";
        import "MixedCase.proto";
        import "with_numbers123.proto";
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let imports = value["imports"].as_array().unwrap();
        assert_eq!(imports[0], "simple/path.proto");
        assert_eq!(imports[1], "path/with/multiple/segments.proto");
        assert_eq!(imports[2], "path_with_underscore.proto");
        assert_eq!(imports[3], "path.with.dots.proto");
        assert_eq!(imports[4], "path-with-dashes.proto");
        assert_eq!(imports[5], "UPPERCASE.proto");
        assert_eq!(imports[6], "MixedCase.proto");
        assert_eq!(imports[7], "with_numbers123.proto");
        Ok(())
    }

    // Testing full_ident rule
    #[test]
    fn test_full_ident_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        package com.example.project.v1;
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["package"], "com.example.project.v1");
        Ok(())
    }

    // Testing syntax rule
    #[test]
    fn test_syntax_rule() -> Result<(), ParserError> {
        let input = r#"syntax = "proto3";"#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["syntax"], "proto3");
        Ok(())
    }

    // Testing package rule
    #[test]
    fn test_package_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        package test.v1.api;
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["package"], "test.v1.api");
        Ok(())
    }

    // Testing import rule
    #[test]
    fn test_import_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        import "other.proto";
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let imports = value["imports"].as_array().unwrap();
        assert_eq!(imports[0], "other.proto");
        Ok(())
    }

    // Testing field_rule
    #[test]
    fn test_field_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        message FieldRuleTest {
            string single = 1;
            repeated string repeated_field = 2;
            optional string optional_field = 3;
            required string required_field = 4;
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let fields = &value["messages"][0]["fields"];
        assert_eq!(fields[0]["repeated"], false);
        assert_eq!(fields[1]["repeated"], true);
        // Note: optional and required are handled according to your implementation
        Ok(())
    }

    // Testing primitive_type rule
    #[test]
    fn test_primitive_type_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        message PrimitiveTypes {
            double double_field = 1;
            float float_field = 2;
            int32 int32_field = 3;
            int64 int64_field = 4;
            uint32 uint32_field = 5;
            uint64 uint64_field = 6;
            sint32 sint32_field = 7;
            sint64 sint64_field = 8;
            fixed32 fixed32_field = 9;
            fixed64 fixed64_field = 10;
            sfixed32 sfixed32_field = 11;
            sfixed64 sfixed64_field = 12;
            bool bool_field = 13;
            string string_field = 14;
            bytes bytes_field = 15;
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let fields = &value["messages"][0]["fields"];
        let types = [
            "double", "float", "int32", "int64", "uint32", "uint64",
            "sint32", "sint64", "fixed32", "fixed64", "sfixed32",
            "sfixed64", "bool", "string", "bytes"
        ];
        for (i, expected_type) in types.iter().enumerate() {
            assert_eq!(fields[i]["type_name"], *expected_type);
        }
        Ok(())
    }

    // Testing message_type rule
    #[test]
    fn test_message_type_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        service TestService {
            rpc Method1 (Request) returns (Response);
            rpc Method2 (stream StreamingRequest) returns (stream StreamingResponse);
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let methods = &value["services"][0]["methods"];
        assert_eq!(methods[0]["input_type"], "Request");
        assert_eq!(methods[0]["output_type"], "Response");
        assert_eq!(methods[1]["input_type"], "StreamingRequest");
        assert_eq!(methods[1]["output_type"], "StreamingResponse");
        Ok(())
    }

    // Testing message_element rule
    #[test]
    fn test_message_element_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        message TestMessage {
            // Regular field
            string regular_field = 1;

            // Nested enum
            enum NestedEnum {
                UNKNOWN = 0;
                VALUE = 1;
            }

            // Nested message
            message NestedMessage {
                int32 value = 1;
            }

            // Fields using nested types
            NestedEnum enum_field = 2;
            NestedMessage message_field = 3;

            // Empty statement
            ;
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        let message = &value["messages"][0];

        // Check regular field
        assert_eq!(message["fields"][0]["name"], "regular_field");

        // Check nested enum
        assert_eq!(message["nested_enums"][0]["name"], "NestedEnum");

        // Check nested message
        assert_eq!(message["nested_messages"][0]["name"], "NestedMessage");

        // Check fields using nested types
        assert_eq!(message["fields"][1]["name"], "enum_field");
        assert_eq!(message["fields"][1]["type_name"], "NestedEnum");
        assert_eq!(message["fields"][2]["name"], "message_field");
        assert_eq!(message["fields"][2]["type_name"], "NestedMessage");

        Ok(())
    }

    // Testing enum_def rule
    #[test]
    fn test_enum_def_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        enum Status {
            UNKNOWN = 0;
            PENDING = 1;
            ACTIVE = 2;
            INACTIVE = 3;
            DELETED = 4;
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        let enum_def = &value["enums"][0];
        assert_eq!(enum_def["name"], "Status");

        let values = enum_def["values"].as_array().unwrap();
        assert_eq!(values[0]["name"], "UNKNOWN");
        assert_eq!(values[0]["number"], 0);
        assert_eq!(values[1]["name"], "PENDING");
        assert_eq!(values[1]["number"], 1);
        assert_eq!(values[2]["name"], "ACTIVE");
        assert_eq!(values[2]["number"], 2);
        assert_eq!(values[3]["name"], "INACTIVE");
        assert_eq!(values[3]["number"], 3);
        assert_eq!(values[4]["name"], "DELETED");
        assert_eq!(values[4]["number"], 4);

        Ok(())
    }

    // Testing enum_value rule
    #[test]
    fn test_enum_value_rule() -> Result<(), ParserError> {
        let input = r#"
    syntax = "proto3";
    enum TestEnum {
        // Testing different naming conventions for enum values
        SIMPLE = 0;
        COMPOUND_NAME = 1;
        NUMBER_123 = 2;
        LARGE_NUMBER = 100;
    }
"#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        let values = &value["enums"][0]["values"];
        assert_eq!(values[0]["name"], "SIMPLE");
        assert_eq!(values[0]["number"], 0);
        assert_eq!(values[1]["name"], "COMPOUND_NAME");
        assert_eq!(values[1]["number"], 1);
        assert_eq!(values[2]["name"], "NUMBER_123");
        assert_eq!(values[2]["number"], 2);
        assert_eq!(values[3]["name"], "LARGE_NUMBER");
        assert_eq!(values[3]["number"], 100);

        Ok(())
    }

    // Testing service_def rule
    #[test]
    fn test_service_def_rule() -> Result<(), ParserError> {
        let input = r#"
        syntax = "proto3";
        service TestService {
            // Empty service
        }

        service ComplexService {
            rpc Method1 (Request1) returns (Response1);
            rpc Method2 (Request2) returns (Response2);
        }

        service StreamingService {
            rpc Method1 (stream Request1) returns (Response1);
            rpc Method2 (Request2) returns (stream Response2);
            rpc Method3 (stream Request3) returns (stream Response3);
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        let services = value["services"].as_array().unwrap();

        // Test empty service
        assert_eq!(services[0]["name"], "TestService");
        assert!(services[0]["methods"].as_array().unwrap().is_empty());

        // Test complex service
        assert_eq!(services[1]["name"], "ComplexService");
        let complex_methods = services[1]["methods"].as_array().unwrap();
        assert_eq!(complex_methods[0]["name"], "Method1");
        assert_eq!(complex_methods[0]["input_type"], "Request1");
        assert_eq!(complex_methods[0]["output_type"], "Response1");
        assert_eq!(complex_methods[1]["name"], "Method2");
        assert_eq!(complex_methods[1]["input_type"], "Request2");
        assert_eq!(complex_methods[1]["output_type"], "Response2");

        // Test streaming service
        assert_eq!(services[2]["name"], "StreamingService");
        let streaming_methods = services[2]["methods"].as_array().unwrap();
        assert_eq!(streaming_methods[0]["name"], "Method1");
        assert_eq!(streaming_methods[0]["input_type"], "Request1");
        assert_eq!(streaming_methods[0]["output_type"], "Response1");
        assert_eq!(streaming_methods[1]["name"], "Method2");
        assert_eq!(streaming_methods[1]["input_type"], "Request2");
        assert_eq!(streaming_methods[1]["output_type"], "Response2");
        assert_eq!(streaming_methods[2]["name"], "Method3");
        assert_eq!(streaming_methods[2]["input_type"], "Request3");
        assert_eq!(streaming_methods[2]["output_type"], "Response3");

        Ok(())
    }

    // Testing rpc_def rule
    #[test]
    fn test_rpc_def_rule() -> Result<(), ParserError> {
        let input = r#"
    syntax = "proto3";
    service RPCTest {
        // Simple RPC
        rpc Simple (SimpleRequest) returns (SimpleResponse);

        // RPC with empty body
        rpc WithEmptyBody (EmptyRequest) returns (EmptyResponse) {}

        // Streaming RPCs
        rpc ClientStream (stream StreamRequest) returns (StreamResponse);
        rpc ServerStream (StreamRequest) returns (stream StreamResponse);
        rpc BidiStream (stream StreamRequest) returns (stream StreamResponse);
    }
"#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        let methods = &value["services"][0]["methods"];

        // Test simple RPC
        assert_eq!(methods[0]["name"], "Simple");
        assert_eq!(methods[0]["input_type"], "SimpleRequest");
        assert_eq!(methods[0]["output_type"], "SimpleResponse");

        // Test RPC with empty body
        assert_eq!(methods[1]["name"], "WithEmptyBody");
        assert_eq!(methods[1]["input_type"], "EmptyRequest");
        assert_eq!(methods[1]["output_type"], "EmptyResponse");

        // Test streaming RPCs
        assert_eq!(methods[2]["name"], "ClientStream");
        assert_eq!(methods[2]["input_type"], "StreamRequest");
        assert_eq!(methods[2]["output_type"], "StreamResponse");

        assert_eq!(methods[3]["name"], "ServerStream");
        assert_eq!(methods[3]["input_type"], "StreamRequest");
        assert_eq!(methods[3]["output_type"], "StreamResponse");

        assert_eq!(methods[4]["name"], "BidiStream");
        assert_eq!(methods[4]["input_type"], "StreamRequest");
        assert_eq!(methods[4]["output_type"], "StreamResponse");

        Ok(())
    }

    // Testing field rule
    #[test]
    fn test_field_definition_rule() -> Result<(), ParserError> {
        let input = r#"
    syntax = "proto3";
    message FieldTest {
        // Simple fields
        string simple_field = 1;
        int32 another_field = 2;

        // Fields with rules
        repeated string repeated_field = 3;
        optional int32 optional_field = 4;
        required bool required_field = 5;

        // Fields with complex types
        OtherMessage message_field = 6;
        repeated OtherMessage repeated_message = 7;

        // Fields with enum types
        TestEnum enum_field = 8;
        repeated TestEnum repeated_enum = 9;

        // Fields with all primitive types
        bool bool_field = 10;
        string string_field = 11;
        bytes bytes_field = 12;
        double double_field = 13;
        float float_field = 14;
        int32 int32_field = 15;
        int64 int64_field = 16;
        uint32 uint32_field = 17;
        uint64 uint64_field = 18;
        sint32 sint32_field = 19;
        sint64 sint64_field = 20;
        fixed32 fixed32_field = 21;
        fixed64 fixed64_field = 22;
        sfixed32 sfixed32_field = 23;
        sfixed64 sfixed64_field = 24;
    }
"#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        let fields = &value["messages"][0]["fields"];

        // Test simple fields
        assert_eq!(fields[0]["name"], "simple_field");
        assert_eq!(fields[0]["type_name"], "string");
        assert_eq!(fields[0]["tag"], 1);
        assert_eq!(fields[0]["repeated"], false);

        // Test repeated fields
        assert_eq!(fields[2]["name"], "repeated_field");
        assert_eq!(fields[2]["type_name"], "string");
        assert_eq!(fields[2]["repeated"], true);

        // Test message type fields
        assert_eq!(fields[6]["name"], "repeated_message");
        assert_eq!(fields[6]["type_name"], "OtherMessage");
        assert_eq!(fields[6]["repeated"], true);

        // Test enum type fields
        assert_eq!(fields[7]["name"], "enum_field");
        assert_eq!(fields[7]["type_name"], "TestEnum");

        // Test primitive type fields
        let primitive_types = [
            "bool", "string", "bytes", "double", "float",
            "int32", "int64", "uint32", "uint64", "sint32",
            "sint64", "fixed32", "fixed64", "sfixed32", "sfixed64"
        ];

        for (i, type_name) in primitive_types.iter().enumerate() {
            assert_eq!(fields[i + 9]["type_name"], *type_name);
        }

        Ok(())
    }

    // Testing proto_file (root) rule
    #[test]
    fn test_proto_file_rule() -> Result<(), ParserError> {
        let input = r#"
        // File starts with syntax
        syntax = "proto3";

        // Package declaration
        package test.v1;

        // Imports
        import "other.proto";
        import "another.proto";

        // Top-level enum
        enum Status {
            UNKNOWN = 0;
            ACTIVE = 1;
        }

        // Top-level message
        message Test {
            string name = 1;
            Status status = 2;

            // Nested enum
            enum Type {
                UNSPECIFIED = 0;
                NORMAL = 1;
            }

            // Nested message
            message Nested {
                int32 value = 1;
            }

            Type type = 3;
            Nested nested = 4;
        }

        // Service definition
        service TestService {
            rpc Get (Test) returns (Test);
        }
    "#;
        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify syntax
        assert_eq!(value["syntax"], "proto3");

        // Verify package
        assert_eq!(value["package"], "test.v1");

        // Verify imports
        let imports = value["imports"].as_array().unwrap();
        assert_eq!(imports[0], "other.proto");
        assert_eq!(imports[1], "another.proto");

        // Verify top-level enum
        let enums = value["enums"].as_array().unwrap();
        assert_eq!(enums[0]["name"], "Status");
        assert_eq!(enums[0]["values"][0]["name"], "UNKNOWN");
        assert_eq!(enums[0]["values"][1]["name"], "ACTIVE");

        // Verify message and its nested types
        let messages = value["messages"].as_array().unwrap();
        let message = &messages[0];
        assert_eq!(message["name"], "Test");

        // Verify nested enum
        let nested_enums = message["nested_enums"].as_array().unwrap();
        assert_eq!(nested_enums[0]["name"], "Type");

        // Verify nested message
        let nested_messages = message["nested_messages"].as_array().unwrap();
        assert_eq!(nested_messages[0]["name"], "Nested");

        // Verify service
        let services = value["services"].as_array().unwrap();
        assert_eq!(services[0]["name"], "TestService");
        assert_eq!(services[0]["methods"][0]["name"], "Get");

        Ok(())
    }
}