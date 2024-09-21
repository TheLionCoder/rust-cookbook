use serde_json::{json, Error, Value};

pub fn serialize_and_deserialize_json() -> Result<(), Error> {
    let json: &str = r#"{
            "user_id": 103609,
            "verified": true,
            "acces_privileges": ["user", "admin"]
            }"#;

    let parsed: Value = serde_json::from_str(json)?;

    let expected = json!({
        "user_id": 103609,
        "verified": true,
        "acces_privileges": ["user", "admin"]
    });

    assert_eq!(parsed, expected);

    Ok(())
}
