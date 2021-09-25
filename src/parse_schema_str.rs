use combine::EasyParser;
use combine::{
    // between,
    error::ParseError,
    many,
    many1,
    parser::char::char,
    parser::char::{letter, newline, space},
    // satisfy,
    // sep_by,
    Parser,
    Stream,
};

use crate::schema::{Data, DataType, FirestoreDataType, Key, Value};

/**
 * main parser
 */
pub fn parse_schema_str(schema_str: &str) {}

/**
 * <text> Collection Users: User
 * ↓
 * <hash mao> {"Collections": {"Users": "User", "Todos" : "Todo"}}
 */
fn parse_schema_collection(collection_str: &str) {}

/**
 * <text> Document User{} \n Document Todo {}
 * ↓
 * <hash mao> {"Documents": {"User": {}, "Todo" : {}}}
 */
fn parse_schema_document(document_str: &str) {}

/**
 * <text> id: Int
 * ↓
 * <struct> {
 *    key: { name: "id", optional: false },
 *    value: Int
 * }
 */
pub fn parse_key_value<Input>() -> impl Parser<Input, Output = (String, String)>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    (
        many1::<String, _, _>(letter()),
        many::<String, _, _>(space().or(newline())),
        char(':'),
        many::<String, _, _>(space().or(newline())),
        many1::<String, _, _>(letter()),
    )
        .map(|v| (v.0, v.4))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_key_value() {
        assert_eq!(
            Ok((("id".to_string(), "Int".to_string()), "")),
            parse_key_value().easy_parse("id: Int")
        )
    }
    // let expected = Data {
    //     key: Key {
    //         name: String::from("id"),
    //         optional: false,
    //     },
    //     value: Value::Data(DataType::FirestoreDataType(FirestoreDataType::Int)),
    // };
}
