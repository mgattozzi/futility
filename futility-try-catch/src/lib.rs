use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Block, Ident, Token, Type,
};

#[proc_macro]
/// `try_` is a macro to use `try/catch` blocks in Rust until they're
/// actually implemented in the language
///
/// Sometimes you want to scope errors to a given block and handle any error in
/// there in one specific way. This is particularly useful if you want to
/// handle failure cases that are recoverable within the function such as
/// retrying an HTTP request. The macro can be used in two different ways:
///
/// ```
/// # use futility_try_catch::try_;
/// # fn function_that_might_fail() -> Result<(), Box<dyn std::error::Error>> {
/// #   Ok(())
/// # }
/// # fn handle_err() {}
/// use std::error::Error;
/// try_!({
///     function_that_might_fail()?;
/// } catch Box<dyn Error> as err {
///     eprintln!("Oh no an error! {err}");
///     handle_err();
/// });
/// ```
///
/// In this case we try a set of statements that can use ? without returning to
/// the top level function scope, but only to the block it's in. This way we can
/// use it idiomatically like we'd expect. We then state for the catch block
/// what Error type we expect it to be and then the name of the error so we can
/// reference it inside of the catch block. This is how it would be used most
/// often.
///
/// The other way is through assignment of the final block value:
/// ```
/// # use futility_try_catch::try_;
/// # fn function_that_might_fail() -> Result<(), Box<dyn std::error::Error>> {
/// #   Ok(())
/// # }
/// # fn handle_err() {}
/// use std::error::Error;
/// let try_value = try_!({
///     function_that_might_fail()?;
///     "This is returned if it does not fail"
/// } catch Box<dyn Error> as err {
///     eprintln!("Oh no an error! {err}");
///     handle_err();
///     "This is returned if it does fail"
/// });
/// ```
///
/// In this case you must return the same type in each block, but it does let
/// you assign a value from the `try/catch` block if you'd like. Simply omit the
/// semicolon like you would when returning a value in a function.
///
/// ### How it works/expands
/// The macro is actually relatively small in terms of implementation and what
/// it expands out too. This call:
/// ```
/// # use futility_try_catch::try_;
/// # fn function_that_might_fail() -> Result<(), Box<dyn std::error::Error>> {
/// #   Ok(())
/// # }
/// # fn handle_err() {}
/// use std::error::Error;
/// try_!({
///     function_that_might_fail()?;
/// } catch Box<dyn Error> as err {
///     eprintln!("Oh no an error! {err}");
///     handle_err();
/// });
/// ```
///
/// expands out to:
/// ```
/// # use futility_try_catch::try_;
/// # fn function_that_might_fail() -> Result<(), Box<dyn std::error::Error>> {
/// #   Ok(())
/// # }
/// # fn handle_err() {}
/// use std::error::Error;
/// match || -> Result<_, Box<dyn Error>> {
///     Ok({
///         function_that_might_fail()?;
///     })
/// }() {
///     Ok(val) => val,
///     Err(err) => {
///         eprintln!("Oh no an error! {err}");
///         handle_err();
///     }
/// }
/// ```
///
/// This is where the magic is, if we use a closure then we can use `?` inside
/// of it and scope it to only the block of that function. This means we don't
/// automatically return all of the way to the top level function where the
/// macro is invoked and we can handle the error locally! This is however, not
/// the prettiest to look at and might be considered "unidiomatic" Rust. The
/// macro therefore abstracts over this and makes it nicer to work with/look at.
pub fn try_(tokens: TokenStream) -> TokenStream {
    let TryCatchInput {
        try_block,
        catch_block,
        error_ty,
        error_ident,
    } = parse_macro_input!(tokens as TryCatchInput);
    let expanded = quote! {
        match || -> ::std::result::Result<_, #error_ty> {
            ::std::result::Result::Ok(#try_block)
        }() {
          ::std::result::Result::Ok(ret) => ret,
          ::std::result::Result::Err(#error_ident) => #catch_block
       }
    };
    TokenStream::from(expanded)
}

struct TryCatchInput {
    try_block: Block,
    catch_block: Block,
    error_ty: Type,
    error_ident: Ident,
}

impl Parse for TryCatchInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let try_block: Block = input.parse()?;
        let catch: Ident = input.parse()?;
        assert_eq!(catch, "catch");
        let error_ty: Type = input.parse()?;
        let _: Token![as] = input.parse()?;
        let error_ident: Ident = input.parse()?;
        let catch_block: Block = input.parse()?;

        Ok(Self {
            try_block,
            catch_block,
            error_ty,
            error_ident,
        })
    }
}
