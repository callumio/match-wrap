#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, Expr, Pat, Result as SynResult, Token, Type, TypePath,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_str,
};

struct MatchWrapInput {
    trait_type: Type,
    expr: Expr,
    arms: Vec<(Vec<Attribute>, Pat, Expr)>,
}

impl Parse for MatchWrapInput {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let trait_type = input.parse()?;
        input.parse::<Token![;]>()?;

        let expr = input.parse()?;
        input.parse::<Token![;]>()?;

        let mut arms = Vec::new();
        while !input.is_empty() {
            let attrs = input.call(syn::Attribute::parse_outer)?;
            let pat = Pat::parse_single(input)?;
            input.parse::<Token![=>]>()?;
            let arm_expr = input.parse()?;
            arms.push((attrs, pat, arm_expr));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(MatchWrapInput {
            trait_type,
            expr,
            arms,
        })
    }
}

fn match_wrap(input: TokenStream, container_type: &str) -> TokenStream {
    const DIVERGE_ATTR: &str = "diverges";
    let input = parse_macro_input!(input as MatchWrapInput);
    let trait_type = &input.trait_type;
    let container: TypePath =
        parse_str(container_type).expect("Incorrect container type. This is likely a bug.");

    let expr = &input.expr;

    let arms = input.arms.iter().map(|(attrs, pat, arm_expr)| {
        let is_diverging = attrs.iter().any(|attr| attr.path().is_ident(DIVERGE_ATTR));

        if is_diverging {
            quote! {#pat => #arm_expr}
        } else {
            quote! { #pat => #container::new(#arm_expr) as #container<#trait_type> }
        }
    });

    quote! { match #expr { #(#arms,)* } }.into()
}

#[proc_macro]
pub fn match_box(input: TokenStream) -> TokenStream {
    match_wrap(input, "::std::boxed::Box")
}

#[proc_macro]
pub fn match_arc(input: TokenStream) -> TokenStream {
    match_wrap(input, "::std::sync::Arc")
}

#[proc_macro]
pub fn match_rc(input: TokenStream) -> TokenStream {
    match_wrap(input, "::std::rc::Rc")
}
