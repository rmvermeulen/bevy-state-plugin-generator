pub use insta::*;
pub use rstest::*;
pub use speculoos::prelude::*;

#[macro_export]
#[allow(missing_docs)]
macro_rules! set_snapshot_suffix {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_suffix(format!($($expr,)*));
        let _guard = settings.bind_to_scope();
    }
}
