use crate::algorithm::Printer;
use crate::iter::IterDelimited;
use crate::INDENT;
use std::ptr;
use syn::{
    AngleBracketedGenericArguments, AssocConst, AssocType, Constraint, GenericArgument,
    ParenthesizedGenericArguments, Path, PathArguments, PathSegment, QSelf,
};

#[derive(Copy, Clone, PartialEq)]
pub enum PathKind {
    // a::B
    Simple,
    // a::B<T>
    Type,
    // a::B::<T>
    Expr,
}

impl Printer {
    pub fn path(&mut self, path: &Path, kind: PathKind) {
        assert!(!path.segments.is_empty());
        for segment in path.segments.iter().delimited() {
            if !segment.is_first || path.leading_colon.is_some() {
                self.word("::");
            }
            self.path_segment(&segment, kind);
        }
    }

    pub fn path_segment(&mut self, segment: &PathSegment, kind: PathKind) {
        self.ident(&segment.ident);
        self.path_arguments(&segment.arguments, kind);
    }

    fn path_arguments(&mut self, arguments: &PathArguments, kind: PathKind) {
        match arguments {
            PathArguments::None => {}
            PathArguments::AngleBracketed(arguments) => {
                self.angle_bracketed_generic_arguments(arguments, kind);
            }
            PathArguments::Parenthesized(arguments) => {
                self.parenthesized_generic_arguments(arguments);
            }
        }
    }

    fn generic_argument(&mut self, arg: &GenericArgument) {
        match arg {
            #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
            GenericArgument::Lifetime(lifetime) => self.lifetime(lifetime),
            GenericArgument::Type(ty) => self.ty(ty),
            GenericArgument::Const(expr) => self.const_argument(expr),
            GenericArgument::AssocType(assoc) => self.assoc_type(assoc),
            GenericArgument::AssocConst(assoc) => self.assoc_const(assoc),
            GenericArgument::Constraint(constraint) => self.constraint(constraint),
            _ => unimplemented!("unknown GenericArgument"),
        }
    }

    pub fn angle_bracketed_generic_arguments(
        &mut self,
        generic: &AngleBracketedGenericArguments,
        path_kind: PathKind,
    ) {
        if generic.args.is_empty() || path_kind == PathKind::Simple {
            return;
        }

        if path_kind == PathKind::Expr {
            self.word("::");
        }
        self.word("<");
        self.cbox(INDENT);
        self.zerobreak();

        // Print lifetimes before types/consts/bindings, regardless of their
        // order in self.args.
        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        enum Group {
            First,
            Second,
        }
        fn group(arg: &GenericArgument) -> Group {
            match arg {
                #![cfg_attr(all(test, exhaustive), deny(non_exhaustive_omitted_patterns))]
                GenericArgument::Lifetime(_) => Group::First,
                GenericArgument::Type(_)
                | GenericArgument::Const(_)
                | GenericArgument::AssocType(_)
                | GenericArgument::AssocConst(_)
                | GenericArgument::Constraint(_) => Group::Second,
                _ => Group::Second,
            }
        }
        let last = generic.args.iter().max_by_key(|param| group(param));
        for current_group in [Group::First, Group::Second] {
            for arg in &generic.args {
                if group(arg) == current_group {
                    self.generic_argument(arg);
                    self.trailing_comma(ptr::eq(arg, last.unwrap()));
                }
            }
        }

        self.offset(-INDENT);
        self.end();
        self.word(">");
    }

    fn assoc_type(&mut self, assoc: &AssocType) {
        self.ident(&assoc.ident);
        if let Some(generics) = &assoc.generics {
            self.angle_bracketed_generic_arguments(generics, PathKind::Type);
        }
        self.word(" = ");
        self.ty(&assoc.ty);
    }

    fn assoc_const(&mut self, assoc: &AssocConst) {
        self.ident(&assoc.ident);
        if let Some(generics) = &assoc.generics {
            self.angle_bracketed_generic_arguments(generics, PathKind::Type);
        }
        self.word(" = ");
        self.const_argument(&assoc.value);
    }

    fn constraint(&mut self, constraint: &Constraint) {
        self.ident(&constraint.ident);
        if let Some(generics) = &constraint.generics {
            self.angle_bracketed_generic_arguments(generics, PathKind::Type);
        }
        self.ibox(INDENT);
        for bound in constraint.bounds.iter().delimited() {
            if bound.is_first {
                self.word(": ");
            } else {
                self.space();
                self.word("+ ");
            }
            self.type_param_bound(&bound);
        }
        self.end();
    }

    fn parenthesized_generic_arguments(&mut self, arguments: &ParenthesizedGenericArguments) {
        self.cbox(INDENT);
        self.word("(");
        self.zerobreak();
        for ty in arguments.inputs.iter().delimited() {
            self.ty(&ty);
            self.trailing_comma(ty.is_last);
        }
        self.offset(-INDENT);
        self.word(")");
        self.return_type(&arguments.output);
        self.end();
    }

    pub fn qpath(&mut self, qself: &Option<QSelf>, path: &Path, kind: PathKind) {
        let qself = if let Some(qself) = qself {
            qself
        } else {
            self.path(path, kind);
            return;
        };

        assert!(qself.position < path.segments.len());

        self.word("<");
        self.ty(&qself.ty);

        let mut segments = path.segments.iter();
        if qself.position > 0 {
            self.word(" as ");
            for segment in segments.by_ref().take(qself.position).delimited() {
                if !segment.is_first || path.leading_colon.is_some() {
                    self.word("::");
                }
                self.path_segment(&segment, PathKind::Type);
                if segment.is_last {
                    self.word(">");
                }
            }
        } else {
            self.word(">");
        }
        for segment in segments {
            self.word("::");
            self.path_segment(segment, kind);
        }
    }
}
