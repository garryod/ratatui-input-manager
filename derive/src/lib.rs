#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! Macros for [ratatui-input-manager](https://crates.io/crates/ratatui-input-manager)

use darling::{FromAttributes, FromMeta, ast::NestedMeta};
use itertools::MultiUnzip;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, ExprLit, ImplItem, ItemImpl, Lit, Meta, MetaNameValue, Path, parse_macro_input,
    parse_quote, parse2, spanned::Spanned,
};

#[derive(FromMeta)]
struct KeyMapAttrs {
    #[darling(default)]
    backend: Backend,
}

impl KeyMapAttrs {
    fn parse_attrs(attrs: TokenStream) -> darling::Result<Self> {
        KeyMapAttrs::from_list(&NestedMeta::parse_meta_list(attrs)?)
    }
}

#[derive(Default, FromMeta)]
enum Backend {
    #[cfg_attr(all(feature = "crossterm",), default)]
    Crossterm,
    #[cfg_attr(all(not(feature = "crossterm"), feature = "termion",), default)]
    Termion,
    #[cfg_attr(
        all(
            not(feature = "crossterm"),
            not(feature = "termion"),
            feature = "termwiz"
        ),
        default
    )]
    Termwiz,
}

impl Backend {
    fn key_type(&self) -> Path {
        match self {
            Backend::Crossterm => parse_quote!(::crossterm::event::KeyCode),
            Backend::Termion => parse_quote!(::termion::event::Key),
            Backend::Termwiz => parse_quote!(::termwiz::input::KeyCode),
        }
    }

    fn event_type(&self) -> Path {
        match self {
            Backend::Crossterm => parse_quote!(::crossterm::event::Event),
            Backend::Termion => parse_quote!(::termion::event::Event),
            Backend::Termwiz => parse_quote!(::termwiz::input::InputEvent),
        }
    }

    fn match_clause(&self, key_code: &Expr) -> TokenStream {
        match self {
            Backend::Crossterm => quote! {
                ::crossterm::event::Event::Key(
                    ::crossterm::event::KeyEvent {
                        code: ::crossterm::event::#key_code,
                        kind: ::crossterm::event::KeyEventKind::Press,
                        ..
                    }
                )
            },
            Backend::Termion => quote! {
                ::termion::event::Event::Key(
                    ::termion::event::#key_code
                )
            },
            Backend::Termwiz => quote! {
                ::termwiz::input::InputEvent::Key(
                    ::termwiz::input::KeyEvent {
                        key: ::termwiz::input::#key_code,
                        modifiers: ::termwiz::input::Modifiers::NONE,
                    }
                )
            },
        }
    }
}

#[derive(FromAttributes)]
#[darling(attributes(keybind), forward_attrs)]
struct KeyBindAttrs {
    #[darling(multiple)]
    pressed: Vec<syn::Expr>,
    attrs: Vec<syn::Attribute>,
}

/// Generate an implementation of [`ratatui_input_manager::KeyMap`], dispatching input events to
/// the appropriate methods according to the attributes provided
#[proc_macro_attribute]
pub fn keymap(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = match KeyMapAttrs::parse_attrs(attrs.into()) {
        Ok(args) => args,
        Err(err) => return err.write_errors().into(),
    };
    let input = parse_macro_input!(input as ItemImpl);
    match keymap_impl(args, input) {
        Ok((original_impl, keymap_impl)) => TokenStream::from_iter([
            original_impl.to_token_stream(),
            keymap_impl.to_token_stream(),
        ])
        .into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn keymap_impl(args: KeyMapAttrs, input: ItemImpl) -> syn::Result<(ItemImpl, ItemImpl)> {
    let ItemImpl { self_ty, items, .. } = input;

    let (keybinds, orig_impls) = items
        .into_iter()
        .map(|item| match item {
            ImplItem::Fn(mut item_fn) => {
                let KeyBindAttrs { pressed, attrs } =
                    KeyBindAttrs::from_attributes(&item_fn.attrs)?;
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
        .map(|(fn_name, pressed, description)| {
            let description = match description {
                Some(description) => quote! {Some(#description)},
                None => quote! {None},
            };
            (fn_name, pressed, description)
        })
        .multiunzip();

    let key_type = args.backend.key_type();
    let event_type = args.backend.event_type();
    let match_clauses = key_codes
        .iter()
        .map(|key_codes| {
            key_codes
                .iter()
                .map(|key_code| args.backend.match_clause(key_code))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let keymap_impl = parse_quote! {
        impl ::ratatui_input_manager::KeyMap::<#key_type, #event_type> for #self_ty {
            const KEYBINDS: &'static [::ratatui_input_manager::KeyBind::<#key_type>] = &[
                #(
                    ::ratatui_input_manager::KeyBind::<#key_type> {
                        keys: &[#(#key_codes) , *],
                        description: #descriptions,
                    },
                )*
            ];

            fn handle(&mut self, event: &#event_type) {
                match event {
                    #(
                        #(
                            #match_clauses
                        ) | * => self.#fn_names(),
                    )*
                    _ => {}
                }
            }
        }
    };

    Ok((orig_impl, keymap_impl))
}

#[cfg(test)]
mod tests {
    use super::{Backend, KeyMapAttrs, keymap_impl};
    use pretty_assertions::assert_eq;
    use prettyplease::unparse;
    use quote::quote;
    use syn::{Item, ItemImpl, parse_quote, parse2};

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
    fn test_generated_impl_crossterm() {
        let args = KeyMapAttrs {
            backend: Backend::Crossterm,
        };
        let input = parse_quote! {
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
        };
        let (orig_impl, keymap_impl) = keymap_impl(args, input).unwrap();
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
        let expected_keymap = parse_quote! {
            impl ::ratatui_input_manager::KeyMap::<::crossterm::event::KeyCode, ::crossterm::event::Event> for Foo {
                const KEYBINDS: &'static [::ratatui_input_manager::KeyBind::<::crossterm::event::KeyCode>] = &[
                    ::ratatui_input_manager::KeyBind::<::crossterm::event::KeyCode> {
                        keys: &[KeyCode::Esc, KeyCode::Char('q')],
                        description: None,
                    },
                    ::ratatui_input_manager::KeyBind::<::crossterm::event::KeyCode> {
                        keys: &[KeyCode::Char('a')],
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
                        ) |
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
        };

        assert_eq!(format_item(expected_orig), format_item(orig_impl));
        assert_eq!(
            format_item::<ItemImpl>(expected_keymap),
            format_item(keymap_impl)
        );
    }

    #[test]
    fn test_generated_impl_termion() {
        let args = KeyMapAttrs {
            backend: Backend::Termion,
        };
        let input = parse_quote! {
            impl Foo {
                #[keybind(pressed=Key::Esc)]
                #[keybind(pressed=Key::Char('q'))]
                fn bar(&mut self) {
                    todo!()
                }

                /// The second keybind
                #[keybind(pressed=Key::Char('a'))]
                fn baz(&mut self) {
                    todo!()
                }
            }
        };
        let (orig_impl, keymap_impl) = keymap_impl(args, input).unwrap();
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
        let expected_keymap = parse_quote! {
            impl ::ratatui_input_manager::KeyMap::<::termion::event::Key, ::termion::event::Event> for Foo {
                const KEYBINDS: &'static [::ratatui_input_manager::KeyBind::<::termion::event::Key>] = &[
                    ::ratatui_input_manager::KeyBind::<::termion::event::Key> {
                        keys: &[Key::Esc, Key::Char('q')],
                        description: None,
                    },
                    ::ratatui_input_manager::KeyBind::<::termion::event::Key> {
                        keys: &[Key::Char('a')],
                        description: Some("The second keybind"),
                    }
                ];

                fn handle(&mut self, event: &::termion::event::Event) {
                    match event {
                        ::termion::event::Event::Key(
                            ::termion::event::Key::Esc
                        ) |
                        ::termion::event::Event::Key(
                            ::termion::event::Key::Char('q')
                        ) => self.bar(),
                        ::termion::event::Event::Key(
                            ::termion::event::Key::Char('a')
                        ) => self.baz(),
                        _ => {}
                    }
                }
            }
        };

        assert_eq!(format_item(expected_orig), format_item(orig_impl));
        assert_eq!(
            format_item::<ItemImpl>(expected_keymap),
            format_item(keymap_impl)
        );
    }

    #[test]
    fn test_generated_impl_termwiz() {
        let args = KeyMapAttrs {
            backend: Backend::Termwiz,
        };
        let input = parse_quote! {
            impl Foo {
                #[keybind(pressed=KeyCode::Escape)]
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
        };
        let (orig_impl, keymap_impl) = keymap_impl(args, input).unwrap();
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
        let expected_keymap = parse_quote! {
            impl ::ratatui_input_manager::KeyMap::<::termwiz::input::KeyCode, ::termwiz::input::InputEvent> for Foo {
                const KEYBINDS: &'static [::ratatui_input_manager::KeyBind::<::termwiz::input::KeyCode>] = &[
                    ::ratatui_input_manager::KeyBind::<::termwiz::input::KeyCode> {
                        keys: &[KeyCode::Escape, KeyCode::Char('q')],
                        description: None,
                    },
                    ::ratatui_input_manager::KeyBind::<::termwiz::input::KeyCode> {
                        keys: &[KeyCode::Char('a')],
                        description: Some("The second keybind"),
                    }
                ];

                fn handle(&mut self, event: &::termwiz::input::InputEvent) {
                    match event {
                        ::termwiz::input::InputEvent::Key(
                            ::termwiz::input::KeyEvent {
                                key: ::termwiz::input::KeyCode::Escape,
                                modifiers: ::termwiz::input::Modifiers::NONE,
                            }
                        ) |
                        ::termwiz::input::InputEvent::Key(
                            ::termwiz::input::KeyEvent {
                                key: ::termwiz::input::KeyCode::Char('q'),
                                modifiers: ::termwiz::input::Modifiers::NONE,
                            }
                        ) => self.bar(),
                        ::termwiz::input::InputEvent::Key(
                            ::termwiz::input::KeyEvent {
                                key: ::termwiz::input::KeyCode::Char('a'),
                                modifiers: ::termwiz::input::Modifiers::NONE,
                            }
                        ) => self.baz(),
                        _ => {}
                    }
                }
            }
        };

        assert_eq!(format_item(expected_orig), format_item(orig_impl));
        assert_eq!(
            format_item::<ItemImpl>(expected_keymap),
            format_item(keymap_impl)
        );
    }
}
