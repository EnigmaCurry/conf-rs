use heck::ToKebabCase;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Error, Expr, ExprLit, Fields, Lit, LitStr, Meta,
    parse_macro_input,
};

mod proc_macro_options;
use proc_macro_options::GenConfStruct;

mod subcommand_proc_macro_options;
use subcommand_proc_macro_options::GenSubcommandsEnum;

pub(crate) mod util;

/// Derive a `Conf` implementation for an item with `#[conf(...)]` attributes
#[proc_macro_derive(Conf, attributes(conf, arg))]
pub fn conf(input: TokenStream1) -> TokenStream1 {
    let input: DeriveInput = parse_macro_input!(input);
    derive_conf(&input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

fn derive_conf(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let ident = &input.ident;

    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => {
            let gener = GenConfStruct::new(ident, &input.attrs, fields)?;
            let conf_impl = gener.gen_conf_impl(&input.generics)?;
            let maybe_serde = gener.maybe_gen_conf_serde_impl(&input.generics)?;
            let maybe_test = gener.maybe_gen_test_fn(&input.generics)?;

            Ok(quote! {
                #conf_impl

                #maybe_serde

                #maybe_test
            })
        }

        _ => Err(Error::new(
            ident.span(),
            "#[derive(Conf)] is only supported on structs with named fields",
        )),
    }
}

/// Derive a `Subcommands` implementation for an item with `#[conf(...)]` attributes
#[proc_macro_derive(Subcommands, attributes(conf, arg))]
pub fn subcommands(input: TokenStream1) -> TokenStream1 {
    let input: DeriveInput = parse_macro_input!(input);
    derive_subcommands(&input)
        .unwrap_or_else(|error| error.to_compile_error())
        .into()
}

fn derive_subcommands(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let ident = &input.ident;

    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let gener = GenSubcommandsEnum::new(ident, &input.attrs, variants.into_iter())?;
            gener.gen_all(&input.generics)
        }

        _ => Err(Error::new(
            ident.span(),
            "#[derive(Subcommands)] is only supported on enums",
        )),
    }
}

#[proc_macro_derive(ValueEnum, attributes(conf))]
pub fn derive_value_enum(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let Data::Enum(data) = input.data else {
        return syn::Error::new_spanned(enum_ident, "ValueEnum can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut entries = Vec::new();

    for v in data.variants {
        if !matches!(v.fields, Fields::Unit) {
            return syn::Error::new_spanned(v.ident, "ValueEnum only supports unit variants")
                .to_compile_error()
                .into();
        }

        let default_name = v.ident.to_string().to_kebab_case();

        let mut name_lit = LitStr::new(&default_name, v.ident.span());
        for attr in &v.attrs {
            if !attr.path().is_ident("conf") {
                continue;
            }
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let lit: LitStr = meta.value()?.parse()?;
                    name_lit = lit;
                    return Ok(());
                }
                Ok(())
            });
        }

        let mut help: Option<LitStr> = None;
        for attr in &v.attrs {
            if !attr.path().is_ident("doc") {
                continue;
            }

            if let Meta::NameValue(nv) = &attr.meta {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(ls), ..
                }) = &nv.value
                {
                    let line = ls.value().trim().to_string();
                    if !line.is_empty() {
                        help = Some(LitStr::new(&line, ls.span()));
                        break;
                    }
                }
            }
        }

        let help_tokens = match help {
            Some(h) => quote! { Some(#h) },
            None => quote! { None },
        };

        entries.push(quote! {
            ::conf::PossibleValue {
                name: #name_lit,
                aliases: &[],
                help: #help_tokens,
            }
        });
    }

    let expanded = quote! {
        impl #enum_ident {
            /// Enumerated values for completions/help (static-friendly).
            pub const POSSIBLE_VALUES: &'static [::conf::PossibleValue] = &[
                #(#entries),*
            ];
        }

        impl ::conf::ValueEnum for #enum_ident {
            fn possible_values() -> &'static [::conf::PossibleValue] {
                Self::POSSIBLE_VALUES
            }
        }
    };

    expanded.into()
}
