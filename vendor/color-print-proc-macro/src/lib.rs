//! This internal library provides the procedural macros needed by the crate [`color-print`].
//!
//! [`color-print`]: https://crates.io/crates/color-print

extern crate proc_macro;

#[macro_use]
mod util;
#[cfg(not(feature = "terminfo"))]
mod ansi;
#[cfg(not(feature = "terminfo"))]
mod ansi_constants;
mod color_context;
mod error;
mod format_args;
mod parse;
#[cfg(feature = "terminfo")]
mod terminfo;
mod untagged;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};

/// The same as `format!()`, but parses color tags.
///
/// #### Example
///
/// ```
/// # use color_print_proc_macro::cformat;
/// let s: String = cformat!("A <g>green</> word, {}", "placeholders are allowed");
/// assert_eq!(s, "A \u{1b}[32mgreen\u{1b}[39m word, placeholders are allowed");
/// ```
#[proc_macro]
#[cfg(not(feature = "terminfo"))]
pub fn cformat(input: TokenStream) -> TokenStream {
    get_macro("format", input)
}

/// The same as `format!()`, but parses color tags.
#[proc_macro]
#[cfg(feature = "terminfo")]
pub fn cformat(input: TokenStream) -> TokenStream {
    get_macro("format", input)
}

/// The same as `print!()`, but parses color tags.
///
/// #### Example
///
/// ```
/// # use color_print_proc_macro::cprint;
/// cprint!("A <g>green</> word, {}", "placeholders are allowed");
/// ```
#[proc_macro]
#[cfg(not(feature = "terminfo"))]
pub fn cprint(input: TokenStream) -> TokenStream {
    get_macro("print", input)
}

/// The same as `print!()`, but parses color tags.
#[proc_macro]
#[cfg(feature = "terminfo")]
pub fn cprint(input: TokenStream) -> TokenStream {
    get_macro("print", input)
}

/// The same as `println!()`, but parses color tags.
///
/// #### Example
///
/// ```
/// # use color_print_proc_macro::cprintln;
/// cprintln!("A <g>green</> word, {}", "placeholders are allowed");
/// ```
#[proc_macro]
#[cfg(not(feature = "terminfo"))]
pub fn cprintln(input: TokenStream) -> TokenStream {
    get_macro("println", input)
}

/// The same as `println!()`, but parses color tags.
#[proc_macro]
#[cfg(feature = "terminfo")]
pub fn cprintln(input: TokenStream) -> TokenStream {
    get_macro("println", input)
}

/// Colorizes a string literal, without formatting the `format!`-like placeholders.
///
/// * Accepts only one argument;
/// * Will panic if feature `terminfo` is activated.
///
/// #### Example
///
/// ```
/// # use color_print_proc_macro::cstr;
/// let s: &str = cstr!("A <g>green</> word");
/// assert_eq!(s, "A \u{1b}[32mgreen\u{1b}[39m word");
/// ```
#[cfg(not(feature = "terminfo"))]
#[proc_macro]
pub fn cstr(input: TokenStream) -> TokenStream {
    crate::ansi::get_cstr(input)
        .unwrap_or_else(|err| err.to_token_stream())
        .into()
}

/// Removes all the color tags from the given string literal.
///
/// Accepts only one argument.
///
/// #### Example
///
/// ```
/// # use color_print_proc_macro::untagged;
/// let s: &str = untagged!("A <g>normal</> word");
/// assert_eq!(s, "A normal word");
/// ```
#[proc_macro]
pub fn untagged(input: TokenStream) -> TokenStream {
    crate::untagged::get_untagged(input)
        .unwrap_or_else(|err| err.to_token_stream())
        .into()
}

/// Colorizes a string literal, without formatting the `format!`-like placeholders.
///
/// * Accepts only one argument;
/// * Will panic if feature `terminfo` is activated.
#[cfg(feature = "terminfo")]
#[proc_macro]
pub fn cstr(_: TokenStream) -> TokenStream {
    panic!("Macro cstr!() cannot be used with terminfo feature")
}

/// Renders a whole processed macro.
fn get_macro(macro_name: &str, input: TokenStream) -> TokenStream {
    #[cfg(not(feature = "terminfo"))]
    let format_args = crate::ansi::get_format_args(input);
    #[cfg(feature = "terminfo")]
    let format_args = crate::terminfo::get_format_args(input);

    let format_args = format_args.unwrap_or_else(|err| err.to_token_stream());
    let macro_name = util::ident(macro_name);
    (quote! { #macro_name!(#format_args) }).into()
}
