#![feature(box_syntax)]
#![feature(plugin_registrar)]
#![feature(rustc_private)]
#![feature(quote)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

mod lang;
mod marshall;
mod traits;
mod util;

use rustc_plugin::Registry;

use syntax::ptr::P;
use syntax::ast::{Item, ItemKind, MetaItem, Name, Attribute};
use syntax::ext::base::{ExtCtxt,Annotatable};
use syntax::codemap::Span;
use syntax::ext::base::SyntaxExtension;
use syntax::ext::build::AstBuilder;
use syntax::feature_gate::AttributeType;
use syntax::codemap::DUMMY_SP;

// TODO: Warn when exporting something private

/// We should only export things with `#[plug]`.
pub fn should_export(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path == "plug")
}

#[plugin_registrar]
pub fn registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        Name::intern("pluggable"),
        SyntaxExtension::MultiModifier(Box::new(expand_pluggable)),
    );

    reg.register_attribute("pluggable".into(), AttributeType::Normal);
    reg.register_attribute("plug".into(), AttributeType::Whitelisted);
}

/// If the #[pluggable] attribute is on the struct, it is plain old data with no methods.
fn expand_pluggable_struct(ecx: &mut ExtCtxt, item: &Item) -> Vec<Annotatable> {
    let ty = ecx.ty_ident(DUMMY_SP, item.ident);

    let mut items = Vec::new();
    items.push(traits::implement_pluggable_fields(ecx, &ty, &item));
    items.push(traits::implement_pluggable(ecx, &ty));
    items
}

fn expand_unknown_item(ecx: &mut ExtCtxt, sp: Span) {
    ecx.span_err(sp, "only structs and impls can be pluggable".into());
}

fn expand_pluggable(ecx: &mut ExtCtxt, sp: Span, _meta_item: &MetaItem, item: Annotatable) -> Vec<Annotatable> {
    let mut items = Vec::new();
    match item {
        Annotatable::Item(inner_item) => match inner_item.node.clone() {
            ItemKind::Struct(..) => {
                // Push the original
                items.push(Annotatable::Item(inner_item.clone()));
                items.extend(expand_pluggable_struct(ecx, &inner_item))
            },
            ItemKind::Impl(unsafety,polarity,defaultness,generics,tref, ty, impl_items) => {
                let pluggable_impl_items: Vec<_> = impl_items.iter().cloned()
                    .filter(|impl_item| should_export(&impl_item.attrs))
                    .collect();

                // Create function stubs for marshalling.
                let new_impl_items =
                    marshall::create_marshalls(ecx, &pluggable_impl_items);

                // Push the 'impl PluggableMethods' trait impl.
                items.push(traits::implement_pluggable_methods(
                        ecx, &ty, &pluggable_impl_items));

                let impl_items = impl_items.into_iter().chain(new_impl_items).collect();

                // Push the original item with the new impl items attached.
                items.push(Annotatable::Item(P(Item {
                    node: ItemKind::Impl(unsafety, polarity, defaultness, generics, tref, ty, impl_items),
                    ..(*inner_item).clone()
                })));
            },
            _ => {
                expand_unknown_item(ecx, sp);
            },
        },
        _ => {
            expand_unknown_item(ecx, sp);
        },
    }

    items
}

