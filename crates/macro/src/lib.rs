use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(SourceObject)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as DeriveInput);
    impl_source_object(&parsed)
}

fn impl_source_object(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SourceObject for #name {
            fn source_location(&self) -> SourceLocation {
                self.loc
            }
        
            fn source_span(&self) -> SourceSpan {
                self.span
            }
        }
    };
    gen.into()
}
