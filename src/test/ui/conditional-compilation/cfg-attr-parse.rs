// Parse `cfg_attr` with varying numbers of attributes and trailing commas

// compile-pass

// Zero attributes
#[cfg_attr(all(),)]
struct A0YC;

// One attribute, no trailing comma
#[cfg_attr(all(), must_use)]
struct A1NC;

// One attribute, trailing comma
#[cfg_attr(all(), must_use,)]
struct A1YC;

// Two attributes, no trailing comma
#[cfg_attr(all(), must_use, deprecated)]
struct A2NC;

// Two attributes, trailing comma
#[cfg_attr(all(), must_use, deprecated,)]
struct A2YC;

fn main() {}
