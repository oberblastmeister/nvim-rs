use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
  fold::Fold, parse_quote as pq, GenericArgument, TypeArray, TypePath,
  TypeReference, TypeSlice, TypeTuple,
};

pub struct AddLifetime {
  lifetime: syn::Lifetime,
}

impl AddLifetime {
  pub fn new(lifetime: &str) -> AddLifetime {
    AddLifetime {
      lifetime: syn::Lifetime::new(lifetime, Span::call_site()),
    }
  }
}

impl Fold for AddLifetime {
  fn fold_type(&mut self, mut node: syn::Type) -> syn::Type {
    match &mut node {
      syn::Type::Reference(TypeReference { lifetime, elem, .. }) => {
        *lifetime = Some(self.lifetime.clone());
        *elem = Box::new(self.fold_type(*elem.clone()));
      }
      syn::Type::Path(TypePath { path, .. }) => {
        *path = self.fold_path(path.clone());
      }
      syn::Type::Slice(TypeSlice { elem, .. }) => {
        **elem = self.fold_type(*elem.clone());
      }
      syn::Type::Tuple(TypeTuple { elems, .. }) => {
        for elem in elems.iter_mut() {
          *elem = self.fold_type(elem.clone());
        }
      }
      syn::Type::Array(TypeArray { elem, .. }) => {
        **elem = self.fold_type(*elem.clone());
      }
      _ => (),
    }
    node
  }

  fn fold_path(&mut self, node: syn::Path) -> syn::Path {
    fn fold_path2(
      add: &mut AddLifetime,
      mut node: syn::Path,
    ) -> Option<syn::Path> {
      let last_seg = node.segments.iter_mut().last()?;

      match last_seg.arguments {
        syn::PathArguments::None => (),
        syn::PathArguments::AngleBracketed(ref mut angle_bracketed) => {
          for generic_arg in angle_bracketed.args.iter_mut() {
            if let GenericArgument::Type(ref mut tipe) = generic_arg {
              *tipe = add.fold_type(tipe.clone())
            }
          }
        }
        _ => (),
      }

      Some(node)
    }

    fold_path2(self, node.clone()).unwrap_or(node)
  }
}

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

#[cfg(test)]
mod tests {
  use super::*;

  fn check(tipe: syn::Type, expected: syn::Type) {
    assert_eq!(AddLifetime::new("'a").fold_type(tipe), expected)
  }

  #[test]
  fn primitive_fold() {
    check(pq! { &str }, pq! { &'a str });
    check(pq! { &u32 }, pq! { &'a u32 });
    check(pq! { bool }, pq! { bool });
  }

  #[test]
  fn path_fold() {
    check(pq! { std::vec::Vec<&str> }, pq! { std::vec::Vec<&'a str> });
    check(pq! { Vec<&str> }, pq! { Vec<&'a str> });
    check(pq! { Option<&str> }, pq! { Option<&'a str> });
  }

  #[test]
  fn array_fold() {
    check(pq! { &[bool] }, pq! { &'a [bool] });
    check(pq! { &[&str] }, pq! { &'a [&'a str] });
    check(pq! { [&str; 10] }, pq! { [&'a str; 10] })
  }

  #[test]
  fn tuple_fold() {
    check(pq! { &(&str, &str) }, pq! { &'a (&'a str, &'a str) })
  }
}
