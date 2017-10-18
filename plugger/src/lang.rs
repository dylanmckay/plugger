//! Utilities for marshalling values between Rust and other languages.

use syntax::ast;
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::ptr::P;

/// A language that supports marshalling.
pub trait Language : Sync {
    fn name(&self) -> &'static str;
    fn plugger_crate_name(&self) -> &'static str;

    /// Gets the path the `Marshall` for the language.
    fn marshall_path(&self, ecx: &mut ExtCtxt) -> ast::Path;
    /// Gets the marshall type for the language.
    fn marshall_ty(&self, ecx: &mut ExtCtxt) -> P<ast::Ty> {
        let path = self.marshall_path(ecx);
        ecx.ty_path(path)
    }

    /// Gets a `Path` to the value type used by the language.
    fn value_path(&self, ecx: &mut ExtCtxt) -> ast::Path;
    /// Gets the value type for the language.
    fn value_ty(&self, ecx: &mut ExtCtxt) -> P<ast::Ty> {
        let path = self.value_path(ecx);
        ecx.ty_path(path)
    }

    /// Sets a default return value for all pluggable functions.
    ///
    /// If this is set, then the language-specific marshall function will
    /// gain a return value if they didn't have one already, and this value
    /// will be returned in that case.
    fn default_return_value(&self, ecx: &mut ExtCtxt) -> Option<P<ast::Expr>>;
}

/// A list of all languages.
pub static LANGUAGES: &'static [&'static Language] = &[
    #[cfg(feature = "ruby")] &ruby::Ruby,
];

/// The Ruby programming language.
#[cfg(feature = "ruby")]
mod ruby {
    use super::Language;
    use syntax::ast;
    use syntax::codemap::DUMMY_SP;
    use syntax::ext::base::ExtCtxt;
    use syntax::ext::build::AstBuilder;
    use syntax::ptr::P;

    pub struct Ruby;

    impl Language for Ruby {
        fn name(&self) -> &'static str { "ruby" }
        fn plugger_crate_name(&self) -> &'static str { "plugger_ruby" }

        fn marshall_path(&self, ecx: &mut ExtCtxt) -> ast::Path {
            ecx.path_global(DUMMY_SP, vec![
                ast::Ident::from_str(self.plugger_crate_name()),
                ast::Ident::from_str("Marshall")])
        }

        fn value_path(&self, ecx: &mut ExtCtxt) -> ast::Path {
            ecx.path_global(DUMMY_SP, vec![
                ast::Ident::from_str(self.plugger_crate_name()),
                ast::Ident::from_str("Value"),
            ])
        }

        // We want all values without return values to automatically gain a return
        // value of `nil`.
        fn default_return_value(&self, ecx: &mut ExtCtxt) -> Option<P<ast::Expr>> {
            let nil_path = ecx.path_global(DUMMY_SP, vec![
                ast::Ident::from_str(self.plugger_crate_name()),
                ast::Ident::from_str("Value"),
                ast::Ident::from_str("nil"),
            ]);
            let nil_expr = ecx.expr_path(nil_path);
            Some(ecx.expr_call(DUMMY_SP, nil_expr, Vec::new()))
        }
    }
}

