use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Bundle)]
pub fn derive_bundle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => &f.named,
            _ => panic!("Bundle can only be derived for structs with named fields"),
        },
        _ => panic!("Bundle can only be derived for structs"),
    };

    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();

    let expanded = quote! {
        impl ecs_core::Bundle for #name {
            fn component_ids(registry: &mut ecs_core::ComponentRegistry) -> Vec<ecs_core::ComponentId> {
                let mut ids = vec![
                    #( registry.id(TypeId::of::<#types>()), )*
                ];
                ids.sort_unstable();
                ids
            }

            fn insert(self, archetype: &mut ecs_core::Archetype) {
                #( archetype.push::<#types>(self.#idents); )*
            }
        }
    };

    TokenStream::from(expanded)
}
