use std::{
    any::{type_name, type_name_of_val},
    rc::Rc,
};

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum StateConfig {
    Single(String),
    Many(String, Vec<Rc<StateConfig>>),
}

impl StateConfig {
    pub fn name(&self) -> &str {
        match self {
            StateConfig::Single(name) => name,
            StateConfig::Many(name, _) => name,
        }
    }
    pub fn single<S: ToString>(name: S) -> Self {
        Self::Single(name.to_string())
    }
    #[cfg(test)]
    pub fn many<S: ToString, C: Into<Rc<StateConfig>>, V: IntoIterator<Item = C>>(
        name: S,
        variants: V,
    ) -> Self {
        Self::Many(
            name.to_string(),
            variants.into_iter().map(Into::into).collect(),
        )
    }
}

impl StateConfig {
    pub fn add_variant_rev(&mut self, variant: Rc<StateConfig>) {
        let to_add = variant.clone();
        match self {
            StateConfig::Single(name) => {
                *self = StateConfig::Many(name.to_string(), vec![variant]);
            }
            StateConfig::Many(_, variants) => {
                variants.insert(0, variant);
            }
        }
        println!(
            "{}::{}: {:?} (after adding {:?})",
            type_name::<Self>(),
            type_name_of_val(&Self::add_variant_rev),
            self,
            to_add
        );
    }
}
