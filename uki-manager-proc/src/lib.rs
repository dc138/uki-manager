use proc_macro as pm;
use proc_macro2::Span as pm2s;
use syn::parse_macro_input;
use syn::Data as sd;
use syn::Fields as sf;
use syn::Ident as si;

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
                let segments = &match field_ty {
                    syn::Type::Path(ref path) => path,
                    _ => panic!("all fields should e of path type"),
                }
                .path
                .segments;

                if segments.len() != 1 {
                    panic!("nested members should be direct struct types");
                }

                let ty_ident = &segments
                    .first()
                    .expect("invalid type for nested member")
                    .ident;

                let ty_optional_ident = si::new(&format!("{}Option", ty_ident), pm2s::call_site());

                quote::quote! { Option<#ty_optional_ident> }
            } else {
                quote::quote! { Option<#field_ty> }
            };

            quote::quote! { #field_ident: #ty_final }
        });

    let struct_option_ident = si::new(&format!("{}Option", struct_ident), pm2s::call_site());

    let struct_impl = quote::quote! {
        impl #struct_ident {
            pub fn from_str_default(document: &str, default: Self) -> Result<Self, toml::de::Error> {
                let parsed: #struct_option_ident = toml::from_str(document)?;
                Ok(parsed.unwrap_or(default))
            }
        }
    };

    let struct_option_def = quote::quote! {
        #[derive(serde::Deserialize)]
        struct #struct_option_ident {
            #(#fields_option,)*
        }
    };

    let fields_unwrap = fields_info.iter().map(|(field_ident, _, field_nested)| {
        let value = if *field_nested {
            quote::quote! {
                #field_ident: match self.#field_ident {
                    Some(#field_ident) => #field_ident.unwrap_or(default.#field_ident),
                    None => default.#field_ident,
                }
            }
        } else {
            quote::quote! {
                #field_ident: self.#field_ident.unwrap_or(default.#field_ident)
            }
        };

        quote::quote! {
            #value
        }
    });

    let struct_option_impl = quote::quote! {
        impl #struct_option_ident {
            fn unwrap_or(self, default: #struct_ident) -> #struct_ident {
                #struct_ident {
                    #(#fields_unwrap,)*
                }
            }
        }
    };

    pm::TokenStream::from(quote::quote! {
        #struct_impl
        #struct_option_def
        #struct_option_impl
    })
}
