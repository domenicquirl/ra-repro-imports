fn main() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Toggle {
    A,
    B,
}

impl fmt::Display for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Toggle::A => write!(f, "A"),
            Toggle::B => write!(f, "B"),
        }
    }
}
