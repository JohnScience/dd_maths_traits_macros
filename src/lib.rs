use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{TokenTree as TokenTree2, Ident as Ident2, Span as Span2};

#[proc_macro]
pub fn impl_fixed_size_int_subset(ts: TokenStream) -> TokenStream {
    let ts = proc_macro2::TokenStream::from(ts);
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let dd_maths_traits_ident = if crate_name == "dd_maths_traits" {
        Ident2::new("crate", Span2::call_site())
    } else {
        Ident2::new("dd_maths_traits", Span2::call_site())
    };
    let type_token: TokenTree2 = ts.into_iter().next().unwrap().into();
    quote! {
        impl #dd_maths_traits_ident::int::FixedSizeIntSubset for #type_token {}
        impl #dd_maths_traits_ident::int::IntSubset for #type_token {
            const IS_FIXED_SIZE: bool = true;
        }
    }.into()
}

#[proc_macro]
pub fn impl_arbitrary_size_int_subset(ts: TokenStream) -> TokenStream {
    let ts = proc_macro2::TokenStream::from(ts);
    let type_token: TokenTree2 = ts.into_iter().next().unwrap().into();
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let dd_maths_traits_ident = if crate_name == "dd_maths_traits" {
        Ident2::new("crate", Span2::call_site())
    } else {
        Ident2::new("dd_maths_traits", Span2::call_site())
    };
    quote! {
        impl #dd_maths_traits_ident::int::ArbitrarySizeIntSubset for #type_token {}
        impl #dd_maths_traits_ident::int::IntSubset for #type_token {
            const IS_FIXED_SIZE: bool = false;
        }
    }.into()
}