#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! Macros for [ratatui-input-manager](https://crates.io/crates/ratatui-input-manager)

use darling::FromAttributes;
use itertools::MultiUnzip;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, ExprLit, ImplItem, ItemImpl, Lit, Meta, MetaNameValue, parse_macro_input, parse2,
    spanned::Spanned,
};

#[derive(FromAttributes)]
#[darling(attributes(keybind), forward_attrs)]
struct KeybindArgs {
    #[darling(multiple)]
    pressed: Vec<syn::Expr>,
    attrs: Vec<syn::Attribute>,
}

/// Generate an implementation of [`ratatui_input_manager::KeyMap`], dispatching input events to
/// the appropriate methods according to the attributes provided
#[proc_macro_attribute]
pub fn keymap(
    _attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemImpl);
    match keymap_impl(input) {
        Ok((original_impl, keymap_impl)) => TokenStream::from_iter([
            original_impl.to_token_stream(),
            keymap_impl.to_token_stream(),
        ])
        .into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn keymap_impl(input: ItemImpl) -> syn::Result<(ItemImpl, ItemImpl)> {
    let ItemImpl { self_ty, items, .. } = input;

    let (keybinds, orig_impls) = items
        .into_iter()
        .map(|item| match item {
            ImplItem::Fn(mut item_fn) => {
                let KeybindArgs { pressed, attrs } = KeybindArgs::from_attributes(&item_fn.attrs)?;
                let doc = attrs.iter().find_map(|attr| {
                    if let Meta::NameValue(MetaNameValue { path, value, .. }) = &attr.meta
                        && path.is_ident("doc")
                    {
                        match value {
                            Expr::Lit(ExprLit {
                                lit: Lit::Str(doc), ..
                            }) => Some(doc.value().trim().to_string()),
                            _ => None,
                        }
                    } else {
                        None
                    }
                });
                item_fn.attrs = attrs;
                Ok(((item_fn.sig.ident.clone(), pressed, doc), item_fn))
            }
            _ => Err(syn::Error::new(
                item.span(),
                "Only function definitions are permitted with a keymap",
            )),
        })
        .collect::<Result<(Vec<_>, Vec<_>), _>>()?;

    let orig_impl = parse2(quote::quote! {
        impl #self_ty {
            #(#orig_impls)*
        }
    })
    .unwrap();

    let (fn_names, key_codes, descriptions): (Vec<_>, Vec<_>, Vec<_>) = keybinds
        .into_iter()
        .flat_map(|(fn_name, pressed, description)| {
            let description = match description {
                Some(description) => quote! {Some(#description)},
                None => quote! {None},
            };
            pressed
                .into_iter()
                .map(move |key| (fn_name.clone(), key, description.clone()))
        })
        .multiunzip();

    let keymap_impl = parse2(quote::quote! {
        impl ::ratatui_input_manager::KeyMap for #self_ty {
            const KEYBINDS: &'static [::ratatui_input_manager::KeyBind] = &[
                #(
                    ::ratatui_input_manager::KeyBind {
                        key: ::crossterm::event::#key_codes,
                        description: #descriptions,
                    },
                )*
            ];

            fn handle(&mut self, event: &::crossterm::event::Event) {
                match event {
                    #(
                        ::crossterm::event::Event::Key(
                            ::crossterm::event::KeyEvent {
                                code: ::crossterm::event::#key_codes,
                                kind: ::crossterm::event::KeyEventKind::Press,
                                ..
                            }
                        ) => self.#fn_names(),
                    )*
                    _ => {}
                }
            }
        }
    })
    .unwrap();

    Ok((orig_impl, keymap_impl))
}

#[cfg(test)]
mod tests {
    use crate::keymap_impl;
    use pretty_assertions::assert_eq;
    use prettyplease::unparse;
    use quote::quote;
    use syn::{Item, ItemImpl, parse2};

    fn format_item<I>(item: I) -> String
    where
        I: Into<Item>,
    {
        let file = syn::File {
            attrs: vec![],
            items: vec![item.into()],
            shebang: None,
        };
        unparse(&file)
    }

    #[test]
    fn test_generated_impl() {
        let input = parse2(quote! {
            impl Foo {
                #[keybind(pressed=KeyCode::Esc)]
                #[keybind(pressed=KeyCode::Char('q'))]
                fn bar(&mut self) {
                    todo!()
                }

                /// The second keybind
                #[keybind(pressed=KeyCode::Char('a'))]
                fn baz(&mut self) {
                    todo!()
                }
            }
        })
        .unwrap();
        let (orig_impl, keymap_impl) = keymap_impl(input).unwrap();
        let expected_orig = parse2::<ItemImpl>(quote! {
            impl Foo {
                fn bar(&mut self) {
                    todo!()
                }

                /// The second keybind
                fn baz(&mut self) {
                    todo!()
                }
            }
        })
        .unwrap();
        let expected_keymap = parse2::<ItemImpl>(quote! {
            impl ::ratatui_input_manager::KeyMap for Foo {
                const KEYBINDS: &'static [::ratatui_input_manager::KeyBind] = &[
                    ::ratatui_input_manager::KeyBind {
                        key: ::crossterm::event::KeyCode::Esc,
                        description: None,
                    },
                    ::ratatui_input_manager::KeyBind {
                        key: ::crossterm::event::KeyCode::Char('q'),
                        description: None,
                    },
                    ::ratatui_input_manager::KeyBind {
                        key: ::crossterm::event::KeyCode::Char('a'),
                        description: Some("The second keybind"),
                    }
                ];

                fn handle(&mut self, event: &::crossterm::event::Event) {
                    match event {
                        ::crossterm::event::Event::Key(
                            ::crossterm::event::KeyEvent {
                                code: ::crossterm::event::KeyCode::Esc,
                                kind: ::crossterm::event::KeyEventKind::Press,
                                ..
                            }
                        ) => self.bar(),
                        ::crossterm::event::Event::Key(
                            ::crossterm::event::KeyEvent {
                                code: ::crossterm::event::KeyCode::Char('q'),
                                kind: ::crossterm::event::KeyEventKind::Press,
                                ..
                            }
                        ) => self.bar(),
                        ::crossterm::event::Event::Key(
                            ::crossterm::event::KeyEvent {
                                code: ::crossterm::event::KeyCode::Char('a'),
                                kind: ::crossterm::event::KeyEventKind::Press,
                                ..
                            }
                        ) => self.baz(),
                        _ => {}
                    }
                }
            }
        })
        .unwrap();

        assert_eq!(format_item(expected_orig), format_item(orig_impl));
        assert_eq!(format_item(expected_keymap), format_item(keymap_impl));
    }
}
