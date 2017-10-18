use syntax::ast::{self, Expr, ImplItem, Item, ItemKind, Ty, VariantData};
use syntax::ext::base::{ExtCtxt,Annotatable};
use syntax::ext::build::AstBuilder;
use syntax::codemap::DUMMY_SP;
use syntax::ptr::P;

use {marshall, util};

pub fn implement_pluggable(ecx: &mut ExtCtxt, ty: &P<Ty>) -> Annotatable {
    let ty_path = util::ty_path(ty);
    let ty_name = util::ty_name_str(ty).as_str();

    let dummy_const = ecx.ident_of(&format!("_IMPL_PLUGGABLE_FOR_{}", ty.id));

    Annotatable::Item(quote_item!(ecx,
        #[allow(non_upper_case_globals)]
        const $dummy_const: () = {
            extern crate plugger_core as _plugger;

            impl _plugger::Pluggable for $ty_path {
                fn name(&self) -> &'static str { $ty_name }
            }
        };
    ).unwrap())
}

pub fn implement_pluggable_fields(ecx: &mut ExtCtxt, ty: &P<Ty>, item: &Item) -> Annotatable {
    let ty_path = util::ty_path(ty);
    let dummy_const = ecx.ident_of(&format!("_IMPL_PLUGGABLE_FIELDS_FOR_{}", ty.id));

    let fields = pluggable_struct_fields(ecx, item);
    let fields_body = ecx.expr_vec(DUMMY_SP, fields);

    Annotatable::Item(quote_item!(ecx,
        #[allow(non_upper_case_globals)]
        const $dummy_const: () = {
            extern crate plugger_core as _plugger;

            impl _plugger::PluggableFields for $ty_path {
                fn pluggable_fields(&self) -> Vec<_plugger::Field> {
                    $fields_body.iter().cloned().collect()
                }
            }
        };
    ).unwrap())
}

pub fn implement_pluggable_methods(ecx: &mut ExtCtxt, ty: &P<Ty>, impl_items: &[ImplItem]) -> Annotatable {
    let ty_path = util::ty_path(ty);
    let dummy_const = ecx.ident_of(&format!("_IMPL_PLUGGABLE_METHODS_FOR_{}", ty.id));

    let methods = pluggable_struct_methods(ecx, ty, impl_items);
    let methods_body = ecx.expr_vec(DUMMY_SP, methods);

    Annotatable::Item(quote_item!(ecx,
        #[allow(non_upper_case_globals)]
        const $dummy_const: () = {
            extern crate plugger_core as _plugger;

            impl _plugger::PluggableMethods for $ty_path {
                fn pluggable_methods(&self) -> Vec<_plugger::Method> {
                    $methods_body.iter().cloned().collect()
                }
            }
        };
    ).unwrap())
}

fn pluggable_struct_fields(_ecx: &mut ExtCtxt, struct_item: &Item) -> Vec<P<Expr>> {
    match struct_item.node {
        ItemKind::Struct(VariantData::Struct(ref _fields, _), _) => {
            Vec::new() // FIXME: unimplemented
        },
        _ => unreachable!(),
    }
}

fn pluggable_struct_methods(ecx: &mut ExtCtxt, ty: &P<Ty>, impl_items: &[ImplItem]) -> Vec<P<Expr>> {
    impl_items.iter().filter_map(|impl_item| {
        match impl_item.node {
            ast::ImplItemKind::Method(ref sig, _) => {
                let mut method_path = util::ty_path(ty).clone();
                method_path.segments.push(ast::PathSegment {
                    identifier: impl_item.ident,
                    span: DUMMY_SP,
                    parameters: None,
                });

                let method_expr = ecx.expr_path(method_path);
                let method_name = ecx.expr_str(DUMMY_SP, impl_item.ident.name);

                let is_static = !sig.decl.has_self();

                let return_type = match sig.decl.output {
                    ast::FunctionRetTy::Default(..) => {
                        ecx.expr_none(DUMMY_SP)
                    },
                    ast::FunctionRetTy::Ty(ref ty) => {
                        let expr_str = ecx.expr_str(DUMMY_SP, util::ty_name_str(ty));
                        ecx.expr_some(DUMMY_SP, expr_str)
                    },
                };

                let mut inputs = sig.decl.inputs.iter();

                // Eat 'self' if it exists.
                if !is_static { inputs.next().unwrap(); }

                let parameters = ecx.expr_vec(DUMMY_SP, inputs.filter_map(|arg| {
                    let name_expr = if let ast::PatKind::Ident(_, ref spanned_ident, _) = arg.pat.node {
                        ecx.expr_str(DUMMY_SP, spanned_ident.node.name)
                    } else {
                        ecx.span_err(arg.pat.span, "parameter names must be simple idents");
                        return None;
                    };

                    let ty_expr = ecx.expr_str(DUMMY_SP, util::ty_name_str(&arg.ty));

                    Some(quote_expr!(ecx,
                        _plugger::Parameter {
                            name: $name_expr.to_owned(),
                            ty: $ty_expr,
                        }
                    ))
                }).collect());

                let lang_marshalls = marshall::lang_marshalls(ecx, &impl_item).into_iter().map(|(lang,marshall_path)| {
                    quote_expr!(ecx, ($lang, $marshall_path as *mut _))
                }).collect();
                let lang_marshalls = ecx.expr_vec(DUMMY_SP, lang_marshalls);

                Some(quote_expr!(ecx,
                    _plugger::Method {
                        method_pointer: $method_expr as *mut _,
                        lang_marshalls: $lang_marshalls.to_vec(),
                        name: $method_name,
                        parameters: $parameters.iter().cloned().collect(),
                        ret: $return_type,
                        is_static: $is_static,
                    }
                ))
            },
            _ => {
                ecx.span_err(impl_item.span, "only works on methods");
                None
            }
        }

    }).collect()
}

