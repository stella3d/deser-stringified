use serde::{de::{DeserializeOwned, Visitor}, Deserializer};

/// deserialize a stringified field embedded in other data using a custom
/// parser, allowing you to parse JSON, YAML, or any other serde-compatible
/// format.
pub fn deser_stringified_format<'de, D, T, F, E>(deserializer: D, parser: F) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
    F: Fn(&str) -> Result<T, E>,
    E: std::fmt::Display,
{
    struct StringifiedFormatVisitor<T, F, E> {
        parser: F,
        marker: std::marker::PhantomData<T>,
        error_marker: std::marker::PhantomData<E>,
    }

    impl<'de, T, F, E> Visitor<'de> for StringifiedFormatVisitor<T, F, E>
    where
        T: serde::de::DeserializeOwned,
        F: Fn(&str) -> Result<T, E>,
        E: std::fmt::Display,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing formatted data")
        }

        fn visit_str<A>(self, value: &str) -> Result<Self::Value, A>
        where
            A: serde::de::Error,
        {
            (self.parser)(value).map_err(A::custom)
        }

        fn visit_borrowed_str<A>(self, value: &'de str) -> Result<Self::Value, A>
        where
            A: serde::de::Error,
        {
            (self.parser)(value).map_err(A::custom)
        }
    }

    deserializer.deserialize_str(StringifiedFormatVisitor {
        parser,
        marker: std::marker::PhantomData,
        error_marker: std::marker::PhantomData,
    })
}



pub fn deser_stringified_json<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: DeserializeOwned,
{
    // closure is necessary to solve lifetime issues
    deser_stringified_format(deserializer, |s| serde_json::from_str(s))
}

pub fn deser_stringified_yaml<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: DeserializeOwned,
{
    deser_stringified_format(deserializer, |s| serde_yaml::from_str(s))
}



#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct Metadata {
        key: i32,
    }

    #[test]
    fn json_parsing() {
        #[derive(Deserialize)]
        struct Example<T: DeserializeOwned> {
            #[serde(deserialize_with = "deser_stringified_json")]
            data: T,
        }

        let json_str = r#"{"data": "{\"key\": 1}"}"#;
        let parsed: Example<serde_json::Value> = serde_json::from_str(json_str).unwrap();
        assert_eq!(parsed.data, serde_json::json!({"key": 1}));

        let struct_parsed: Example<Metadata> = serde_json::from_str(json_str).unwrap();
        assert_eq!(struct_parsed.data.key, 1);
    }

    #[test]
    fn yaml_parsing() {
        #[derive(Deserialize)]
        struct Example<T: DeserializeOwned> {
            #[serde(deserialize_with = "deser_stringified_yaml")]
            data: T,
        }

        let yaml_str = r#"
data: "key: 1"
"#;
        let parsed: Example<serde_yaml::Value> = serde_yaml::from_str(yaml_str).unwrap();

        let mut expected_mapping = serde_yaml::Mapping::new();
        expected_mapping.insert(
            serde_yaml::Value::String("key".to_string()),
            serde_yaml::Value::Number(1.into()),
        );
        let expected = serde_yaml::Value::Mapping(expected_mapping);
        assert_eq!(parsed.data, expected);

        let struct_parsed: Example<Metadata> = serde_yaml::from_str(yaml_str).unwrap();
        assert_eq!(struct_parsed.data.key, 1);
    }
}
