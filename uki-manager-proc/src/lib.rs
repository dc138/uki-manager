use proc_macro as pm;
use proc_macro2::Span as pm2s;
use quote::quote;
use syn::parse_macro_input;
use syn::Data as sd;
use syn::Fields as sf;
use syn::Ident as si;
use syn::Meta as sm;

#[proc_macro_derive(TomlParseWithDefault, attributes(default))]
pub fn toml_parse_with_default(item: pm::TokenStream) -> pm::TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    let fields = match input.data {
        sd::Struct(syn::DataStruct {
            fields: sf::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("this method should only be called for structs"),
    };

    let fields_option = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;

        quote::quote! { #vis #ident: Option<#ty> }
    });

    let fields_init_option = fields.iter().map(|f| {
        let ident = &f.ident;

        quote::quote! { #ident: config.#ident.unwrap_or(default.#ident) }
    });

    let fields_init_default = fields.iter().map(|f| {
        let ident = &f.ident;

        let default_value = f
            .attrs
            .iter()
            .find_map(|attr| match attr.meta {
                sm::List(syn::MetaList {
                    ref path,
                    ref tokens,
                    ..
                }) => {
                    match path
                        .segments
                        .first()
                        .expect("attribute path should be non-empty")
                        .ident
                        .to_string()
                        == "default"
                    {
                        true => Some(tokens),
                        false => None,
                    }
                }
                _ => None,
            })
            .expect("all fields should have one default name-value attribute");

        quote! { #ident: #default_value }
    });

    let ident_optional = si::new(&format!("{}Optional", ident), pm2s::call_site());

    pm::TokenStream::from(quote! {
        #[derive(thiserror::Error, Debug)]
        pub enum TomlParseWithDefaultError {
            #[error("io operation")]
            Io(#[from] std::io::Error),

            #[error("toml operation")]
            Toml(#[from] toml::de::Error),
        }

        #[derive(serde::Deserialize)]
        struct #ident_optional {
            #(#fields_option,)*
        }

        impl #ident {
            pub fn parse_with_default(path: String) -> Result<Self, TomlParseWithDefaultError> {
                let default = Self {
                    #(#fields_init_default,)*
                };

                Self::parse_with_custom_default(path, default)
            }

            pub fn parse_with_custom_default(path: String, default: Self) -> Result<Self, TomlParseWithDefaultError> {
                let content = std::fs::read_to_string(path)?;
                let config: #ident_optional = toml::from_str(&content)?;

                Ok(Self {
                    #(#fields_init_option,)*
                })
            }
        }
    })
}

#[proc_macro_derive(TomlFromStrDefault, attributes(nest))]
pub fn toml_from_str_default(item: pm::TokenStream) -> pm::TokenStream {
    let input = parse_macro_input!(item as syn::DeriveInput);
    let struct_ident = input.ident;

    let fields = match input.data {
        sd::Struct(syn::DataStruct {
            fields: sf::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!("this method should only be called for structs"),
    };

    let fields_info: Vec<_> = fields
        .into_iter()
        .map(|field| {
            let field_ident = field
                .ident
                .expect("all fields in the struct should have identifiers");

            let field_ty = field.ty;

            let field_nested = field
                .attrs
                .iter()
                .find_map(|attr| Some(attr.meta.path().get_ident()?.eq("nest")))
                .unwrap_or(false);

            (field_ident, field_ty, field_nested)
        })
        .collect();

    let fields_option = fields_info
        .iter()
        .map(|(field_ident, field_ty, field_nested)| {
            let ty_final = if *field_nested {
                let ty_optional_ident =
                    si::new(&format!("{}Option", field_ident), pm2s::call_site());

                quote::quote! { Option<#ty_optional_ident> }
            } else {
                quote::quote! { Option<#field_ty> }
            };

            quote::quote! { #field_ident: #ty_final }
        });

    let struct_option_ident = si::new(&format!("{}Option", struct_ident), pm2s::call_site());

    let struct_option_init = quote::quote! {
        struct #struct_option_ident {
            #(#fields_option,)*
        }
    };

    let fields_unwrap = fields_info.iter().map(|(field_ident, _, field_nested)| {
        let value = if *field_nested {
            quote::quote! {
                match self.#field_ident {
                    Some(#field_ident) => #field_ident.toml_unwrap_default(&default.#field_ident),
                    None => default.#field_ident.clone(),
                }
            }
        } else {
            quote::quote! {
                self.#field_ident.unwrap_or(default.#field_ident.clone())
            }
        };

        quote::quote! {
            #value
        }
    });

    pm::TokenStream::from(quote::quote! {
        #struct_option_init
    })
}
