use quote::quote;
use syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Entity)]
pub fn entity_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_entity_derive(&ast)
}

pub(crate) fn impl_entity_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl general::domains::models::entity::Entity for #name {
            fn id(&self)->&Id<Self> {
                &self.id
            }
        }

        impl PartialEq for #name{
            fn eq(&self,other:&Self)->bool{
                self.id == other.id
            }
        }
    };
    gen.into()
}
