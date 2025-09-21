use std::borrow::Cow;

use itertools::Itertools;
use lazy_regex::regex;

pub fn normalize_state_name(input: &str) -> String {
    split_by_case(input).join("")
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
