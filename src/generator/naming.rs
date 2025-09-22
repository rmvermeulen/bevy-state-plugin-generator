use std::borrow::Cow;

use itertools::Itertools;
use lazy_regex::regex;

use crate::NamingScheme;
use crate::models::{ParentState, StateNode};

pub trait NormalizeStateName {
    fn normalize_state_name(&self) -> String;
}

impl<S: ToString> NormalizeStateName for S {
    fn normalize_state_name(&self) -> String {
        split_by_case(&self.to_string()).join("")
    }
}

pub fn apply_naming_scheme(
    naming_scheme: NamingScheme,
    node: &StateNode,
    parent: Option<&ParentState>,
) -> String {
    let name = node.name().normalize_state_name();
    match naming_scheme {
        NamingScheme::None => name,
        NamingScheme::Short => {
            if let Some(parent) = parent {
                let parent = parent.state_name();
                format!("{parent}{name}").normalize_state_name()
            } else {
                name
            }
        }
        NamingScheme::Full => {
            if let Some(parent) = parent {
                let parent = parent.ancestral_name();
                format!("{parent}{name}").normalize_state_name()
            } else {
                name
            }
        }
    }
}

fn split_by_case<'s>(input: &'s str) -> Vec<Cow<'s, str>> {
    regex!("(\\d+|[A-Z][a-z]*|^[a-z]+)")
        .captures_iter(input)
        .flat_map(|c| {
            c.iter()
                .filter_map(|x| x.map(|m| m.as_str()))
                .dedup()
                .collect_vec()
        })
        .peekable()
        .batching(|it| {
            let mut word = Vec::new();
            while let Some(next_token) = it.peek() {
                if next_token.len() == 1 {
                    word.push(it.next().unwrap());
                } else if word.is_empty() {
                    return Some(it.next().unwrap().into());
                } else {
                    return Some(word.into_iter().collect::<Cow<str>>());
                }
            }
            (!word.is_empty()).then(|| word.into_iter().collect::<Cow<str>>())
        })
        .dedup()
        .collect_vec()
}

#[cfg(test)]
mod test_case_thing {

    use itertools::Itertools;
    use rstest::rstest;
    use speculoos::assert_that;

    use super::split_by_case;

    #[rstest]
    #[case("example", vec!["example"])]
    #[case("exampleState", vec!["example","State"])]
    #[case("Example", vec!["Example"])]
    #[case("ExampleState", vec!["Example", "State"])]
    #[case("StateState", vec!["State"])]
    #[case("ExampleStateState", vec!["Example","State"])]
    #[case("ExampleStateExampleState", vec!["Example","State","Example","State"])]
    #[case("ABCThing", vec!["ABC", "Thing"])]
    #[case("AB12C", vec!["AB", "12", "C"])]
    #[case("ABCThingExample", vec!["ABC", "Thing", "Example"])]
    #[case("AAABBBCCCorp", vec!["AAABBBCC","Corp"])]
    fn test_split_case(#[case] input: &str, #[case] expected: Vec<&str>) {
        assert_that!(split_by_case(input))
            .is_equal_to(expected.into_iter().map(|s| s.into()).collect_vec());
    }
}
