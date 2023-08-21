use proc_macro as pm;
use proc_macro2::Span as pm2s;
use syn::Data as sd;
use syn::Fields as sf;
use syn::Ident as si;

#[proc_macro_attribute]
pub fn serde_optional(_attr: pm::TokenStream, item: pm::TokenStream) -> pm::TokenStream {
    let ast = syn::parse_macro_input!(item as syn::DeriveInput);

    let name = ast.ident;
    let vis = ast.vis;

    let fields = match ast.data {
        sd::Struct(syn::DataStruct {
            fields: sf::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("you may only call this for structs"),
    };

    let old_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;

        quote::quote! { #vis #name: #ty }
    });

    let new_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let vis = &f.vis;

        quote::quote! { #vis #name: Option<#ty> }
    });

    let new_name = si::new(&format!("{}Option", name), pm2s::call_site());

    pm::TokenStream::from(quote::quote! {
        #vis struct #name {
            #(#old_fields,)*
        }
        #vis struct #new_name {
            #(#new_fields,)*
        }
    })
}
