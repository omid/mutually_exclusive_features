use mutually_exclusive_features::{exactly_one_of, none_or_one_of};

fn main() {
    none_or_one_of!("a", "b", "c");
    none_or_one_of!("a", "b",);
    none_or_one_of!("a");
    none_or_one_of!();

    exactly_one_of!("a", "b", "c");
}