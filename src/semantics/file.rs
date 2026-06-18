use hashbrown::HashSet;

use crate::{
    error::{PawError, PawErrorKind},
    parser::{
        SyntaxNode,
        ast::{decl::Decl, file::File},
    },
    semantics::SematicCheck,
};

impl SematicCheck for File {
    fn check(&self, errors: &mut Vec<PawError>) {
        let mut entry_count = 0;
        let mut seen_names = HashSet::new();

        for decl in &self.decls {
            if let Decl::Func(f) = decl.value() {
                let name = f.name.value();
                if !seen_names.insert(name) {
                    errors.push(PawError::new(
                        PawErrorKind::DuplicateDeclaration(name.clone()),
                        format!("Duplicate function '{}'", name),
                        f.name.loc(),
                    ))
                }

                if f.is_entry {
                    entry_count += 1;

                    if entry_count > 1 {
                        errors.push(PawError::new(
                            PawErrorKind::MultipleEntryPoints,
                            "Only one entry function is allowed".to_string(),
                            decl.loc(),
                        ));
                    }
                }
            }

            // TODO: semantics check on decl "decl.check(errors);"
        }

        if entry_count == 0 {
            errors.push(PawError::new(
                PawErrorKind::MissingEntryPoint,
                "No entry function declared. Use 'func !main'".to_string(),
                self.loc.clone(),
            ));
        }
    }
}
