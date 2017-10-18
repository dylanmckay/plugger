use lang::{LANGUAGES, Language};
use util;

use syntax::ptr::P;
use syntax::ast::{self, Block, BlockCheckMode, DUMMY_NODE_ID, Expr, ExprKind, Ident, ImplItemKind, ImplItem, PatKind, Path, PathSegment, PolyTraitRef, Stmt, StmtKind, ThinVec, TraitBoundModifier, TyParam, TyParamBound};
use syntax::ext::base::ExtCtxt;
use syntax::ext::build::AstBuilder;
use syntax::codemap::DUMMY_SP;

use std::collections::HashMap;

/// Gets a hash map of all of the language-specific marshalls.
pub fn lang_marshalls(ecx: &mut ExtCtxt,
                      impl_item: &ImplItem) -> HashMap<String, Path> {
    LANGUAGES.iter().map(|&lang| {
        (lang.name().to_owned(),
         config::lang_marshall_path(ecx, impl_item.ident, lang))
    }).collect()
}

/// Creates all of the marshalling methods for a set of impl items.
pub fn create_marshalls(ecx: &mut ExtCtxt,
                        impl_items: &[ImplItem]) -> Vec<ImplItem> {
    let mut marshall_fns = Vec::new();

    for impl_item in impl_items.iter() {
        match impl_item.node {
            ImplItemKind::Method(ref sig, ..) => {
                let original_name = impl_item.ident;
                let common_marshall_fn =
                    create_common_marshall(ecx, original_name, sig);
                marshall_fns.push(common_marshall_fn);

                for &lang in LANGUAGES {
                    let lang_marshall_fn = create_lang_marshall(ecx, original_name, sig, lang);
                    marshall_fns.push(lang_marshall_fn);
                }
            },
            _ => (),
        }
    }

    marshall_fns
}

/// Creates a common `<original_name>_marshall` method.
///
/// This can be used to marshall types from any language that has an implementation
/// of the `Marshall` trait.
fn create_common_marshall(ecx: &mut ExtCtxt,
                          original_name: Ident,
                          method_sig: &ast::MethodSig) -> ImplItem {
    let mut marshall_sig = method_sig.clone();

    // Get a path to the `Marshall` trait.
    let marshall_trait_path = Path {
        segments: vec![
            PathSegment::from_ident(Ident::from_str("plugger_core"), DUMMY_SP),
            PathSegment::from_ident(Ident::from_str("Marshall"), DUMMY_SP),
        ],
        span: DUMMY_SP,
    };
    let marshall_trait_ref = PolyTraitRef::new(Vec::new(), marshall_trait_path, DUMMY_SP);

    // Add a type parameter for the module.
    // TODO: consider what happens when there is already a type parameter with this
    // name.
    marshall_sig.generics.ty_params.insert(0, TyParam {
        attrs: ThinVec::new(),
        ident: Ident::from_str("M"),
        id: DUMMY_NODE_ID,
        bounds: vec![TyParamBound::TraitTyParamBound(marshall_trait_ref, TraitBoundModifier::None)],
        default: None,
        span: DUMMY_SP,
    });

    let value_ty_path = ecx.path(DUMMY_SP, vec![
        Ident::from_str("M"), Ident::from_str("Value"),
    ]);
    let value_ty = ecx.ty_path(value_ty_path);

    marshall_sig.decl = util::replace_signature_types(marshall_sig.decl.clone(),
                                                      value_ty);

    // Create expressions to marshall given arguments to the correct arguments.
    let marshalled_args: Vec<_> = method_sig.decl.inputs.iter().map(|arg| {
        if arg.is_self() {
            ecx.expr_self(DUMMY_SP) // leave the self argument untouched
        } else { // map the argument to the correct type
            let arg_name = match arg.pat.node {
                PatKind::Ident(_, ref ident, _) => ident.node,
                _ => unimplemented!(),
            };
            let ty_name = util::ty_name_str(&arg.ty);
            let marshall_fn = Ident::from_str(&format!("to_{}", ty_name).to_lowercase());

            P(quote_expr!(ecx, M::$marshall_fn($arg_name)).unwrap())
        }
    }).collect();

    let original_fn_expr = quote_expr!(ecx, Self::$original_name);

    let call_expr = P(Expr {
        id: DUMMY_NODE_ID,
        span: DUMMY_SP,
        attrs: ThinVec::new(),
        node: ExprKind::Call(original_fn_expr, marshalled_args),
    });

    // Marshall the return value if present.
    let result_expr = match method_sig.decl.output {
        ast::FunctionRetTy::Default(..) => call_expr,
        ast::FunctionRetTy::Ty(ref ty) => {
            let ty_name = util::ty_name_str(&ty);
            let marshall_fn = Ident::from_str(&format!("from_{}", ty_name).to_lowercase());
            quote_expr!(ecx, M::$marshall_fn($call_expr))
        },
    };

    // Add statement to call original function.
    let result_stmt = Stmt {
        id: DUMMY_NODE_ID,
        span: DUMMY_SP,
        node: StmtKind::Expr(result_expr),
    };

    let block = Block {
        id: DUMMY_NODE_ID,
        span: DUMMY_SP,
        rules: BlockCheckMode::Default,
        stmts: vec![result_stmt],
    };

    ImplItem {
        id: DUMMY_NODE_ID,
        span: DUMMY_SP,
        ident: config::common_marshall_name(original_name),
        node: ImplItemKind::Method(marshall_sig, P(block)),
        vis: ast::Visibility::Public,
        defaultness: ast::Defaultness::Final,
        attrs: Vec::new(),
        tokens: None,
    }
}

/// Creates a language-specific marshalling method that calls into the common one.
fn create_lang_marshall(ecx: &mut ExtCtxt,
                        original_name: Ident,
                        method_sig: &ast::MethodSig,
                        lang: &Language) -> ImplItem {
    let mut marshall_sig = method_sig.clone();

    // Replace old parameter types with the language-specific value type.
    marshall_sig.decl = util::replace_signature_types(marshall_sig.decl.clone(), lang.value_ty(ecx));

    let common_marshall_expr = config::common_marshall_expr(ecx, original_name, lang);
    // Create argument list.
    let args = method_sig.decl.inputs.iter().map(|arg| {
        if arg.is_self() {
            ecx.expr_self(DUMMY_SP)
        } else {
            match arg.pat.node {
                ast::PatKind::Ident(_, ident, _) => {
                    ecx.expr_ident(DUMMY_SP, ident.node)
                },
                _ => unimplemented!(),
            }
        }
    }).collect();

    let mut stmts = Vec::new();

    let call_expr = ecx.expr_call(DUMMY_SP, common_marshall_expr, args);
    let call_stmt = ecx.stmt_expr(call_expr);

    stmts.push(call_stmt);

    match method_sig.decl.output {
        // If we have no return type, but the language has a default value, use it.
        ast::FunctionRetTy::Default(..) => if let Some(default_retval) = lang.default_return_value(ecx) {
            let value_ty = lang.value_ty(ecx);

            marshall_sig.decl = util::set_return_type(marshall_sig.decl, value_ty);
            stmts.push(ecx.stmt_expr(default_retval));
        },
        ast::FunctionRetTy::Ty(..) => (),
    }

    let block = ecx.block(DUMMY_SP, stmts);

    ImplItem {
        id: DUMMY_NODE_ID,
        span: DUMMY_SP,
        ident: config::lang_marshall_name(original_name, lang),
        node: ImplItemKind::Method(marshall_sig, block),
        vis: ast::Visibility::Public,
        defaultness: ast::Defaultness::Final,
        attrs: Vec::new(),
        tokens: None,
    }
}

mod config {
    use lang::Language;
    use syntax::ast::{self, Expr, Ident, Path};
    use syntax::codemap::DUMMY_SP;
    use syntax::ext::base::ExtCtxt;
    use syntax::ext::build::AstBuilder;
    use syntax::ptr::P;

    /// Gets the name of the language-independent marshall function.
    pub fn common_marshall_name(original_fn: Ident) -> Ident {
        Ident::from_str(&format!("{}_marshall", original_fn))
    }

    /// Gets the name of the language-specific marshall function.
    pub fn lang_marshall_name(original_fn: Ident, lang: &Language) -> Ident {
        Ident::from_str(&format!("{}_{}", original_fn, lang.name()))
    }

    /// Gets a path to the common marshall function.
    pub fn common_marshall_path(ecx: &mut ExtCtxt, original_fn: Ident, lang: &Language) -> Path {
        let mut path = ecx.path(DUMMY_SP, vec![Ident::from_str("Self"),
                                common_marshall_name(original_fn)]);

        path.segments.last_mut().unwrap().parameters = Some(P(ast::PathParameters::AngleBracketed(
            ast::AngleBracketedParameterData {
                span: DUMMY_SP,
                lifetimes: Vec::new(),
                types: vec![lang.marshall_ty(ecx)],
                bindings: Vec::new(),
            }
        )));
        path
    }

    /// Gets a path to the language-specific marshall function.
    pub fn lang_marshall_path(ecx: &mut ExtCtxt,
                              original_fn: Ident,
                              lang: &Language) -> Path {
        ecx.path(DUMMY_SP, vec![Ident::from_str("Self"),
            lang_marshall_name(original_fn, lang)])
    }

    pub fn common_marshall_expr(ecx: &mut ExtCtxt, original_fn: Ident, lang: &Language) -> P<Expr> {
        let path = common_marshall_path(ecx, original_fn, lang);
        ecx.expr_path(path)
    }
}

