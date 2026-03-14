mod codegen;
mod parse;

use proc_macro2::TokenStream;
use quote::quote;
use syn::parse2;

use parse::MacroInput;

pub fn expand(input: TokenStream) -> syn::Result<TokenStream> {
    let MacroInput { entries } = parse2::<MacroInput>(input)?;

    let expanded: Vec<TokenStream> = entries.iter().map(codegen::emit_entry).collect();

    Ok(quote! {
        #(#expanded)*
    })
}
