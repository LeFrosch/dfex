use quote::{quote, format_ident};
use proc_macro2::TokenStream;

use crate::automata::{Dfa, DfaNode};

pub fn matcher(name: &str, dfa: &Dfa) -> TokenStream {
    let name = format_ident!("{}", name);
    let start_state = dfa.get_start_state().get_id();
    
    let states = dfa.iter().map(transitions);
    let final_states = dfa.iter().map(final_state);

    quote! {
        fn #name(input: &str) -> bool {
            let mut state: usize = #start_state;
            
            for c in input.chars() {
                match state {
                    #(#states,)*
                    _ => return false
                };
            }

            match state {
                #(#final_states,)*
                _ => false
            }
        }
    }
}

fn transitions(node: &DfaNode) -> TokenStream  {
    let id = node.get_id();
    let transitions: Vec<TokenStream> = node.iter().map(|(key, id)| quote! { #key => #id }).collect();

    if transitions.is_empty() {
        quote! { #id => return false }
    } else {
        quote! {
            #id => state = match c {
                #(#transitions,)*
                _ => return false
            }
        }
    }
}

fn final_state(node: &DfaNode) -> TokenStream {
    let id = node.get_id();
    let final_state = node.is_final_state();
    
    quote! { #id => #final_state }
}