#[cfg(test)]
use common::test_utils::arbitrary_with_size;
use derive_getters::Getters;
use deriving_via::DerivingVia;
#[cfg(test)]
use proptest_derive::Arbitrary;
use serde::Deserialize;
use strum_macros::EnumString;
use typed_builder::TypedBuilder;

#[cfg_attr(test, derive(Arbitrary))]
#[derive(DerivingVia, Clone, Copy, Debug, PartialOrd, PartialEq)]
#[deriving(From, Into, Serialize(via: i32))]
pub struct FormId(i32);

#[cfg_attr(test, derive(Arbitrary))]
#[derive(DerivingVia, TypedBuilder, Deserialize, Clone, Getters, Debug, PartialOrd, PartialEq)]
#[deriving(From, Into)]
pub struct FormName {
    #[builder(setter(into))]
    name: String,
}

#[cfg_attr(test, derive(Arbitrary))]
#[derive(TypedBuilder, Getters, Debug, PartialEq)]
pub struct Form {
    id: FormId,
    name: FormName,
    #[cfg_attr(test, proptest(strategy = "arbitrary_with_size(1..100)"))]
    questions: Vec<Question>,
}

#[cfg_attr(test, derive(Arbitrary))]
#[derive(TypedBuilder, Getters, Debug, PartialEq)]
pub struct Question {
    title: String,
    description: String,
    question_type: QuestionType,
    #[cfg_attr(test, proptest(strategy = "arbitrary_with_size(1..100)"))]
    choices: Vec<String>,
}

#[cfg_attr(test, derive(Arbitrary))]
#[derive(Debug, EnumString, PartialOrd, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum QuestionType {
    TEXT,
    SINGLE,
    MULTIPLE,
}

impl TryFrom<String> for QuestionType {
    type Error = errors::domain::DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use std::str::FromStr;
        Self::from_str(&value).map_err(Into::into)
    }
}

#[cfg(test)]
mod test {
    use proptest::{prop_assert_eq, proptest};
    use serde_json::json;
    use test_case::test_case;

    use super::*;

    #[test_case("TEXT"     => Ok(QuestionType::TEXT); "upper: TEXT")]
    #[test_case("text"     => Ok(QuestionType::TEXT); "lower: text")]
    #[test_case("SINGLE" => Ok(QuestionType::SINGLE); "upper: SINGLE")]
    #[test_case("single" => Ok(QuestionType::SINGLE); "lower: single")]
    #[test_case("MULTIPLE" => Ok(QuestionType::MULTIPLE); "upper: MULTIPLE")]
    #[test_case("multiple" => Ok(QuestionType::MULTIPLE); "lower: multiple")]
    fn string_to_question_type(input: &str) -> Result<QuestionType, errors::domain::DomainError> {
        input.to_owned().try_into()
    }

    proptest! {
        #[test]
        fn string_into_from_name(name: String) {
            let form_name: FormName = name.to_owned().into();
            prop_assert_eq!(form_name, FormName::builder().name(name).build());
        }
    }

    proptest! {
        #[test]
        fn serialize_from_id(id: i32) {
            let form_id: FormId = id.into();
            prop_assert_eq!(json!({"id":form_id}).to_string(), format!(r#"{{"id":{id}}}"#));
        }
    }
}
