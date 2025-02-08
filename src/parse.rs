use std::{ops::Deref, rc::Rc, str::Lines};

use crate::model::StateConfig;

fn chomp_item(mut lines: Lines<'_>) -> Option<(Option<(i32, StateConfig)>, Lines<'_>)> {
    if let Some(line) = lines.next() {
        let depth = line.chars().take_while(|c| c == &' ').count();
        assert!(depth % 2 == 0);
        let name = line.trim();
        if name.is_empty() {
            chomp_item(lines)
        } else {
            Some((Some((depth as i32, StateConfig::single(name))), lines))
        }
    } else {
        None
    }
}

pub(crate) fn parse_state_config(text: &str) -> Rc<StateConfig> {
    let root = Rc::new(StateConfig::single("GameState"));
    let mut nodes = vec![(-2, root.clone(), None)];
    let mut parent_indices = vec![0];
    let mut lines = text.lines();
    while let Some((item, rest)) = chomp_item(lines) {
        println!("Item={item:?}");
        if item.is_none() {
            // parsing failed
            break;
        }
        let (depth, item) = item.unwrap();
        let offset = {
            let current_parent_index = *parent_indices.last().unwrap();
            let parent_entry = &nodes[current_parent_index];
            let offset = depth - parent_entry.0;
            println!(
                "Offset={offset:?} depth={depth:?} parent={:?}",
                parent_entry.0
            );
            offset
        };
        match offset {
            0 => {
                println!(
                    "{item:?} is sibling of {:?}",
                    nodes[*parent_indices.last().unwrap()].1
                );
                // sibling, pop previous parent
                parent_indices.pop();

                // push as new parent
                nodes.push((depth, item.into(), Some(*parent_indices.last().unwrap())));
                parent_indices.push(nodes.len() - 1);
            }
            2 => {
                println!("child");
                // child, add to parent
                let current_parent_index = *parent_indices.last().unwrap();

                // push as new parent
                nodes.push((depth, item.into(), Some(current_parent_index)));
                parent_indices.push(nodes.len() - 1);
            }
            _ => {
                println!("ancestor");
                // parent-sibling, pop parent until depth is reached
                while let Some(last) = parent_indices.last() {
                    if nodes[*last].0 < depth {
                        break;
                    }
                    parent_indices.pop();
                }
                // push as new parent
                nodes.push((depth, item.into(), Some(*parent_indices.last().unwrap())));
                parent_indices.push(nodes.len() - 1);
            }
        }
        println!("Nodes={:?}", nodes);

        lines = rest;
    }
    for index in (0..nodes.len()).rev() {
        let parent = nodes[index].2;
        if let Some(parent_index) = parent {
            let parent_node = nodes.get(parent_index).unwrap();
            let mut parent = parent_node.1.deref().clone();
            parent.add_variant_rev(nodes[index].1.clone());
            nodes[parent_index].1 = parent.into();
        }
    }
    nodes[0].1.clone()
}

#[cfg(test)]
mod tests {

    use rstest::rstest;
    use speculoos::prelude::*;

    use super::*;

    #[rstest]
    #[case::empty("", StateConfig::single("GameState"))]
    #[case::whitespace("  ", StateConfig::single("GameState"))]
    #[case::whitespace("  A", StateConfig::many("GameState", [StateConfig::single("A")]))]
    #[case::whitespace("\n  A", StateConfig::many("GameState", [StateConfig::single("A")]))]
    #[case::simple("Simple", StateConfig::many("GameState", [StateConfig::single("Simple")]))]
    #[case::a_then_b("A\nB", StateConfig::many("GameState", [
        StateConfig::single("A"),
        StateConfig::single("B")
    ]))]
    #[case::a_with_b_and_c("A\n  B\n  C", StateConfig::many("GameState", [
        StateConfig::many("A", [
            StateConfig::single("B"),
            StateConfig::single("C")
        ]),
    ]))]
    #[case::a_with_b_with_c("A\n  B\n    C", StateConfig::many("GameState", [
        StateConfig::many("A", [
            StateConfig::many("B", [
                StateConfig::single("C")
            ]),
        ]),
    ]))]
    #[case::a_with_b_followed_by_c("A\n  B\nC", StateConfig::many("GameState", [
        StateConfig::many("A", [ StateConfig::single("B" ) ]),
        StateConfig::single("C")
    ]))]
    fn test_parse_examples(#[case] text: &str, #[case] expected: StateConfig) {
        assert_that!(*parse_state_config(text)).is_equal_to(&expected);
    }

    #[rstest]
    fn test_states_simple_example() {
        let text = [
            "Parent",
            "  Child",
            "    GrandChild",
            "  Child2",
            "Parent2",
            "  Child",
            "    GrandChild",
            "Parent3",
        ]
        .join("\n");
        let result = parse_state_config(&text);
        assert_that!(*result).is_equal_to(StateConfig::many("GameState", [
            StateConfig::many("Parent", [
                StateConfig::many("Child", [StateConfig::single("GrandChild")]),
                StateConfig::single("Child2"),
            ]),
            StateConfig::many("Parent2", [StateConfig::many("Child", [
                StateConfig::single("GrandChild"),
            ])]),
            StateConfig::single("Parent3"),
        ]));
    }

    #[rstest]
    fn test_states_full_example() {
        let text = [
            "Loading",
            "Ready",
            "  Menu",
            "    Main",
            "    Options",
            "  Game",
            "    Playing",
            "    Paused",
            "    GameOver",
        ]
        .join("\n");
        let result = parse_state_config(&text);
        assert_that!(*result).is_equal_to(StateConfig::many("GameState", [
            StateConfig::single("Loading"),
            StateConfig::many("Ready", [
                StateConfig::many("Menu", [
                    StateConfig::single("Main"),
                    StateConfig::single("Options"),
                ]),
                StateConfig::many("Game", [
                    StateConfig::single("Playing"),
                    StateConfig::single("Paused"),
                    StateConfig::single("GameOver"),
                ]),
            ]),
        ]));
    }
}
