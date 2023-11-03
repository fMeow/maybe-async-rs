fn main() {
    if cfg!(all(feature = "syn-1", feature = "syn-2")) {
        panic!("Features `syn-1` and `syn-2` cannot both be enabled.");
    }

    if cfg!(not(any(feature = "syn-1", feature = "syn-2"))) {
        panic!("At least one of `syn-1` and `syn-2` must be enabled.");
    }
}
