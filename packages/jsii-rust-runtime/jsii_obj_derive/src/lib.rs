extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(JsiiObject, attributes(JsiiObjectFQN))]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let _attrs = input.attrs.into_iter();

    // Build the output, possibly using quasi-quotation
    let name = &input.ident;

    let expanded = quote! {
        impl<'a> JsiiObject<'a> for #name<'_> {
            // parse fqn from macro attribute
            // const FQN: &'a str = #fqn;
            fn get_ref(&self) -> ObjRef {
                self.obj_ref.clone()
            }

            fn get_client(&'a mut self) -> &'a mut JsiiClient {
                self.client
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
