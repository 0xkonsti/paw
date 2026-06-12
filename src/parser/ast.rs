pub trait Parse {
    fn parse() -> Self
    where
        Self: Sized;
}

pub struct Ast {}

impl Parse for Ast {
    fn parse() -> Self {
        Self {}
    }
}
