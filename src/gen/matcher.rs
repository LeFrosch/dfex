use quote::{quote, format_ident};
use proc_macro::TokenStream;

use crate::automata::Dfa;

pub fn matcher(name: &str, dfa: &Dfa) -> TokenStream {
    let name = format_ident!("{}", name);
    
    let tokens = quote! {
        fn #name(input: &str) -> bool {
            let mut state: usize = 0;
            
            for c in input.chars() {
                
            }
        
            false
        }
    };
    
    tokens.into()
}