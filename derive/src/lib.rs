#![forbid(unsafe_code)]
#![warn(missing_docs)]
//! Macros for [ratatui-input-manager](https://crates.io/crates/ratatui-input-manager)

use darling::{FromAttributes, FromMeta, ast::NestedMeta};
use itertools::MultiUnzip;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Expr, ExprLit, ImplItem, ItemImpl, Lit, Meta, MetaNameValue, parse_macro_input, parse_quote,
    spanned::Spanned,
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
    fn backend_type(&self) -> Expr {
        match self {
            Self::Crossterm => parse_quote!(::ratatui_input_manager::CrosstermBackend),
            Self::Termion => parse_quote!(::ratatui_input_manager::TermionBackend),
            Self::Termwiz => parse_quote!(::ratatui_input_manager::TermwizBackend),
        }
    }

    fn combine_modifiers(&self, modifiers: &[Expr]) -> Expr {
        match self {
            Self::Crossterm => {
                parse_quote! {::crossterm::event::KeyModifiers::NONE #(.union(#modifiers))*}
            }
            Self::Termion => parse_quote! { () },
            Self::Termwiz => {
                parse_quote! { ::termwiz::input::Modifiers::NONE #(.union(#modifiers))* }
            }
        }
    }

    fn input_event(&self, key_code: &Expr, combined_modifiers: &Expr) -> TokenStream {
        match self {
            Self::Crossterm => {
                quote! {
                    ::crossterm::event::Event::Key(
                        ::crossterm::event::KeyEvent {
                            code: #key_code,
                            modifiers,
                            kind: ::crossterm::event::KeyEventKind::Press,
                            ..
                        }
                    ) if modifiers.contains(#combined_modifiers)
                }
            }
            Self::Termion => quote! {
                ::termion::event::Event::Key(
                    ::termion::event::#key_code
                )
            },
            Self::Termwiz => {
                quote! {
                    ::termwiz::input::InputEvent::Key(
                        ::termwiz::input::KeyEvent {
                            key: #key_code,
                            modifiers,
                        }
                    ) if modifiers.contains(#combined_modifiers)
                }
            }
        }
    }
}

#[derive(Debug, FromMeta)]
struct Pressed {
    key: syn::Expr,
    #[darling(multiple)]
    modifiers: Vec<syn::Expr>,
}

#[derive(FromAttributes)]
#[darling(attributes(keybind), forward_attrs)]
struct KeyBindAttrs {
    #[darling(multiple)]
    pressed: Vec<Pressed>,
    attrs: Vec<syn::Attribute>,
}

/// Generate an implementation of [`ratatui_input_manager::KeyMap`], dispatching input events to
/// the appropriate methods according to the attributes provided
#[allow(rustdoc::broken_intra_doc_links)]
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
                let doc = attrs
                    .iter()
                    .find_map(|attr| {
                        if let Meta::NameValue(MetaNameValue { path, value, .. }) = &attr.meta
                            && path.is_ident("doc")
                            && let Expr::Lit(ExprLit {
                                lit: Lit::Str(doc), ..
                            }) = value
                        {
                            Some(doc.value().trim().to_string())
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| {
                        syn::Error::new(
                            item_fn.sig.ident.span(),
                            "Keybind functions must have a doc comment for the description",
                        )
                    })?;
                item_fn.attrs = attrs;
                Ok(((item_fn.sig.ident.clone(), pressed, doc), item_fn))
            }
            _ => Err(syn::Error::new(
                item.span(),
                "Only function definitions are permitted with a keymap",
            )),
        })
        .collect::<Result<(Vec<_>, Vec<_>), _>>()?;

    let orig_impl = parse_quote! {
        impl #self_ty {
            #(#orig_impls)*
        }
    };

    let (fn_names, match_arms, key_codes, combined_modifiers, descriptions): (
        Vec<_>,
        Vec<Vec<_>>,
        Vec<Vec<_>>,
        Vec<Vec<_>>,
        Vec<_>,
    ) = keybinds
        .into_iter()
        .map(|(fn_name, pressed, description)| {
            let (match_arm, key_codes, combined_modifiers) = pressed
                .into_iter()
                .map(|Pressed { key, modifiers }| {
                    let combined_modifiers = args.backend.combine_modifiers(&modifiers);
                    let match_arm = args.backend.input_event(&key, &combined_modifiers);
                    (match_arm, key, combined_modifiers)
                })
                .multiunzip();
            (
                fn_name,
                match_arm,
                key_codes,
                combined_modifiers,
                description,
            )
        })
        .multiunzip();

    let backend_type = args.backend.backend_type();

    let keymap_impl = parse_quote! {
        impl ::ratatui_input_manager::KeyMap::<#backend_type> for #self_ty {
            const KEYBINDS: &'static [::ratatui_input_manager::KeyBind::<#backend_type>] = &[
                #(
                    ::ratatui_input_manager::KeyBind::<#backend_type> {
                        pressed: &[
                            #(::ratatui_input_manager::KeyPress::<#backend_type> {
                                key: #key_codes,
                                modifiers: #combined_modifiers,
                            }),*
                        ],
                        description: #descriptions,
                    },
                )*
            ];

            fn handle(&mut self, event: &<#backend_type as ::ratatui_input_manager::Backend>::Event) {
                match event {
                    #(#(#match_arms => self.#fn_names(),)*)*
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
                /// The first keybind
                #[keybind(pressed(key=KeyCode::Esc))]
                #[keybind(pressed(key=KeyCode::Char('q')))]
                fn bar(&mut self) {
                    todo!()
                }

                /// The second keybind
                #[keybind(pressed(key=KeyCode::Char('a'), modifiers=KeyModifiers::CONTROL, modifiers=KeyModifiers::SHIFT))]
                fn baz(&mut self) {
                    todo!()
                }
            }
        };
        let (orig_impl, keymap_impl) = keymap_impl(args, input).unwrap();
        let expected_orig = parse2::<ItemImpl>(quote! {
            impl Foo {
                /// The first keybind
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
        let expected_keymap: ItemImpl = parse_quote! {
            impl ::ratatui_input_manager::KeyMap::<::ratatui_input_manager::CrosstermBackend> for Foo {
                const KEYBINDS: &'static [::ratatui_input_manager::KeyBind::<::ratatui_input_manager::CrosstermBackend>] = &[
                    ::ratatui_input_manager::KeyBind::<::ratatui_input_manager::CrosstermBackend> {
                        pressed: &[
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::CrosstermBackend> {
                                key: KeyCode::Esc,
                                modifiers: ::crossterm::event::KeyModifiers::NONE,
                            },
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::CrosstermBackend> {
                                key: KeyCode::Char('q'),
                                modifiers: ::crossterm::event::KeyModifiers::NONE,
                            },
                        ],
                        description: "The first keybind",
                    },
                    ::ratatui_input_manager::KeyBind::<::ratatui_input_manager::CrosstermBackend> {
                        pressed: &[
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::CrosstermBackend> {
                                key: KeyCode::Char('a'),
                                modifiers: ::crossterm::event::KeyModifiers::NONE
                                    .union(KeyModifiers::CONTROL)
                                    .union(KeyModifiers::SHIFT),
                            },
                        ],
                        description: "The second keybind",
                    }
                ];

                fn handle(&mut self, event: &<::ratatui_input_manager::CrosstermBackend as ::ratatui_input_manager::Backend>::Event) {
                    match event {
                        ::crossterm::event::Event::Key(
                            ::crossterm::event::KeyEvent {
                                code: KeyCode::Esc,
                                modifiers,
                                kind: ::crossterm::event::KeyEventKind::Press,
                                ..
                            },
                        ) if modifiers.contains(::crossterm::event::KeyModifiers::NONE) => self.bar(),
                        ::crossterm::event::Event::Key(
                            ::crossterm::event::KeyEvent {
                                code: KeyCode::Char('q'),
                                modifiers,
                                kind: ::crossterm::event::KeyEventKind::Press,
                                ..
                            },
                        ) if modifiers.contains(::crossterm::event::KeyModifiers::NONE) => self.bar(),
                        ::crossterm::event::Event::Key(
                            ::crossterm::event::KeyEvent {
                                code: KeyCode::Char('a'),
                                modifiers,
                                kind: ::crossterm::event::KeyEventKind::Press,
                                ..
                            },
                        ) if modifiers
                            .contains(
                                ::crossterm::event::KeyModifiers::NONE
                                    .union(KeyModifiers::CONTROL)
                                    .union(KeyModifiers::SHIFT),
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
                /// The first keybind
                #[keybind(pressed(key=Key::Esc))]
                #[keybind(pressed(key=Key::Char('q')))]
                fn bar(&mut self) {
                    todo!()
                }

                /// The second keybind
                #[keybind(pressed(key=Key::Char('a')))]
                fn baz(&mut self) {
                    todo!()
                }
            }
        };
        let (orig_impl, keymap_impl) = keymap_impl(args, input).unwrap();
        let expected_orig = parse2::<ItemImpl>(quote! {
            impl Foo {
                /// The first keybind
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
            impl ::ratatui_input_manager::KeyMap::<::ratatui_input_manager::TermionBackend> for Foo {
                const KEYBINDS: &'static [::ratatui_input_manager::KeyBind::<::ratatui_input_manager::TermionBackend>] = &[
                    ::ratatui_input_manager::KeyBind::<::ratatui_input_manager::TermionBackend> {
                        pressed: &[
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::TermionBackend> {
                                key: Key::Esc,
                                modifiers: (),
                            },
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::TermionBackend> {
                                key: Key::Char('q'),
                                modifiers: (),
                            },
                        ],
                        description: "The first keybind",
                    },
                    ::ratatui_input_manager::KeyBind::<::ratatui_input_manager::TermionBackend> {
                        pressed: &[
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::TermionBackend> {
                                key: Key::Char('a'),
                                modifiers: (),
                            },
                        ],
                        description: "The second keybind",
                    }
                ];

                fn handle(&mut self, event: &<::ratatui_input_manager::TermionBackend as ::ratatui_input_manager::Backend>::Event) {
                    match event {
                        ::termion::event::Event::Key(
                            ::termion::event::Key::Esc
                        ) => self.bar(),
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
                /// The first keybind
                #[keybind(pressed(key=KeyCode::Escape))]
                #[keybind(pressed(key=KeyCode::Char('q')))]
                fn bar(&mut self) {
                    todo!()
                }

                /// The second keybind
                #[keybind(pressed(key=KeyCode::Char('a'), modifiers=Modifiers::CTRL, modifiers=Modifiers::SHIFT))]
                fn baz(&mut self) {
                    todo!()
                }
            }
        };
        let (orig_impl, keymap_impl) = keymap_impl(args, input).unwrap();
        let expected_orig = parse2::<ItemImpl>(quote! {
            impl Foo {
                /// The first keybind
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
            impl ::ratatui_input_manager::KeyMap::<::ratatui_input_manager::TermwizBackend> for Foo {
                const KEYBINDS: &'static [::ratatui_input_manager::KeyBind::<::ratatui_input_manager::TermwizBackend>] = &[
                    ::ratatui_input_manager::KeyBind::<::ratatui_input_manager::TermwizBackend> {
                        pressed: &[
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::TermwizBackend> {
                                key: KeyCode::Escape,
                                modifiers: ::termwiz::input::Modifiers::NONE,
                            },
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::TermwizBackend> {
                                key: KeyCode::Char('q'),
                                modifiers: ::termwiz::input::Modifiers::NONE,
                            },
                        ],
                        description: "The first keybind",
                    },
                    ::ratatui_input_manager::KeyBind::<::ratatui_input_manager::TermwizBackend> {
                        pressed: &[
                            ::ratatui_input_manager::KeyPress::<::ratatui_input_manager::TermwizBackend> {
                                key: KeyCode::Char('a'),
                                modifiers: ::termwiz::input::Modifiers::NONE
                                    .union(Modifiers::CTRL)
                                    .union(Modifiers::SHIFT),
                            },
                        ],
                        description: "The second keybind",
                    }
                ];

                fn handle(&mut self, event: &<::ratatui_input_manager::TermwizBackend as ::ratatui_input_manager::Backend>::Event) {
                    match event {
                        ::termwiz::input::InputEvent::Key(
                            ::termwiz::input::KeyEvent {
                                key: KeyCode::Escape,
                                modifiers,
                            },
                        ) if modifiers.contains(::termwiz::input::Modifiers::NONE) => self.bar(),
                        ::termwiz::input::InputEvent::Key(
                            ::termwiz::input::KeyEvent {
                                key: KeyCode::Char('q'),
                                modifiers,
                            },
                        ) if modifiers.contains(::termwiz::input::Modifiers::NONE) => self.bar(),
                        ::termwiz::input::InputEvent::Key(
                            ::termwiz::input::KeyEvent {
                                key: KeyCode::Char('a'),
                                modifiers,
                            },
                        ) if modifiers
                            .contains(
                                ::termwiz::input::Modifiers::NONE
                                    .union(Modifiers::CTRL)
                                    .union(Modifiers::SHIFT),
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
    fn test_missing_doc_comment_error() {
        let args = KeyMapAttrs {
            backend: Backend::Crossterm,
        };
        let input = parse_quote! {
            impl Foo {
                #[keybind(pressed(key=KeyCode::Esc))]
                fn bar(&mut self) {
                    todo!()
                }
            }
        };
        let result = keymap_impl(args, input);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(
            err.to_string(),
            "Keybind functions must have a doc comment for the description"
        );
    }
}
