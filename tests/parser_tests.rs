#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use proto_parser::{ParserError, Proto};

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

        Ok(())
    }

    #[test]
    fn test_parse_enum() -> Result<(), ParserError> {
        let input = r#"
            syntax = "proto3";

            enum Status {
                UNKNOWN = 0;
                ACTIVE = 1;
                INACTIVE = 2;
            }
        "#;

        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["enums"][0]["name"], "Status");
        assert_eq!(value["enums"][0]["values"][0]["name"], "UNKNOWN");
        assert_eq!(value["enums"][0]["values"][0]["number"], 0);

        Ok(())
    }

    #[test]
    fn test_parse_service() -> Result<(), ParserError> {
        let input = r#"
            syntax = "proto3";

            service TestService {
                rpc GetData(GetRequest) returns (GetResponse);
            }
        "#;

        let json = Proto::parse(input)?;
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(value["services"][0]["name"], "TestService");
        assert_eq!(value["services"][0]["methods"][0]["name"], "GetData");
        assert_eq!(value["services"][0]["methods"][0]["input_type"], "GetRequest");

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

        assert_eq!(value["messages"][0]["name"], "Outer");
        assert_eq!(value["messages"][0]["nested_messages"][0]["name"], "Inner");

        Ok(())
    }
}