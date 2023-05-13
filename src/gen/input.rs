use syn::{Ident, Token, LitStr};
use syn::parse::{Parse, ParseStream};

pub struct Input {
    pub ident: String,
    pub regex: String,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let regex: LitStr = input.parse()?;
        
        Ok(Self { ident: ident.to_string(), regex: regex.value() })
    }
}