use std::time::{SystemTime, UNIX_EPOCH};

use lazy_static::*;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

lazy_static! {
    static ref NOW: SystemTime = SystemTime::now();
}

/// The number of milliseconds since midnight 1 January 1970.
#[proc_macro]
pub fn milliseconds(input: TokenStream) -> TokenStream {
    std::mem::drop(input);

    let since_epoch = NOW
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    let lit = syn::LitInt::new(&since_epoch.to_string(), Span::call_site());
    return quote! { (#lit as u128) }.into();
}

/// The number of microseconds since midnight 1 January 1970.
#[proc_macro]
pub fn microseconds(input: TokenStream) -> TokenStream {
    std::mem::drop(input);

    let since_epoch = NOW
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros();

    let lit = syn::LitInt::new(&since_epoch.to_string(), Span::call_site());
    return quote! { (#lit as u128) }.into();
}

/// The number of nanoseconds since midnight 1 January 1970.
#[proc_macro]
pub fn nanoseconds(input: TokenStream) -> TokenStream {
    std::mem::drop(input);

    let since_epoch = NOW
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos();

    let lit = syn::LitInt::new(&since_epoch.to_string(), Span::call_site());
    return quote! { (#lit as u128) }.into();
}

/// The number of seconds since midnight 1 January 1970.
#[proc_macro]
pub fn seconds(input: TokenStream) -> TokenStream {
    std::mem::drop(input);

    let since_epoch = NOW
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let lit = syn::LitInt::new(&since_epoch.to_string(), Span::call_site());
    return quote! { (#lit as u64) }.into();
}
