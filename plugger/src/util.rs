use syntax::ptr::P;
use syntax::ast::{self, Name, Ty,TyKind,Path};

/// Gets the path to a type.
pub fn ty_path(ty: &P<Ty>) -> &Path {
    if let TyKind::Path(_, ref path) = ty.node {
        path
    } else {
        unimplemented!();
    }
}

/// Gets the name of a type.
pub fn ty_name_str(ty: &P<Ty>) -> Name {
    match ty.node {
        TyKind::Path(_, ref path) => {
            path.segments.iter().last().unwrap().identifier.name
        },
        _ => Name::intern("<unknown type>"),
    }
}

/// Replaces all parameter types in a function declaration.
pub fn replace_signature_types(decl: P<ast::FnDecl>, new_ty: P<ast::Ty>) -> P<ast::FnDecl> {
    P(ast::FnDecl {
        output: match decl.output {
            ast::FunctionRetTy::Default(sp) => ast::FunctionRetTy::Default(sp),
            ast::FunctionRetTy::Ty(..) => ast::FunctionRetTy::Ty(new_ty.clone()),
        },
        variadic: decl.variadic,
        inputs: decl.inputs.iter().map(|arg| {
            if arg.is_self() {
                arg.clone() // leave untouched
            } else {
                ast::Arg {
                    ty: new_ty.clone(),
                    ..arg.clone()
                }
            }
        }).collect()
    })
}

/// Replaces all parameter types in a function declaration.
pub fn set_return_type(decl: P<ast::FnDecl>, new_ty: P<ast::Ty>) -> P<ast::FnDecl> {
    P(ast::FnDecl {
        output: ast::FunctionRetTy::Ty(new_ty.clone()),
        inputs: decl.inputs.clone(),
        variadic: decl.variadic,
    })
}

