use crate::{
    error::{PawError, PawResults},
    parser::Parser,
};

mod file;

pub trait SematicCheck {
    fn check(&self, errors: &mut Vec<PawError>);
}

pub struct SematicAnaylser;

impl SematicAnaylser {
    pub fn check(parser: &Parser) -> PawResults<()> {
        let mut errors = Vec::new();

        parser.ast.root.check(&mut errors);

        if errors.is_empty() {
            return Ok(());
        }

        Err(errors)
    }
}
