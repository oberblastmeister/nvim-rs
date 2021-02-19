use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse_quote as pq;

/// Represent optional quoting, just a newtype wrapper around option
pub struct MaybeQuote(pub Option<TokenStream>);

impl ToTokens for MaybeQuote {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match &self.0 {
      Some(new_tokens) => tokens.extend(new_tokens.clone()),
      None => (),
    }
  }
}

pub fn is_ext_type(tipe: &syn::Type) -> bool {
  tipe == &pq! { Window<W> }
    || tipe == &pq! { Buffer<W> }
    || tipe == &pq! { Tabpage<W> }
}
