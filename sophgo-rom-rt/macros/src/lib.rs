use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, parse_macro_input, ItemFn, ReturnType, Type, Visibility};

/// ROM runtime function entry.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "#[entry] attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    if f.sig.inputs.len() != 1 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[entry]` function without rom peripherals should not include any parameter",
        )
        .to_compile_error()
        .into();
    }
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && matches!(f.sig.output, ReturnType::Type(_, ref t) if matches!(t.as_ref(), &Type::Never(_)));

    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "`#[entry]` function must have signature `[unsafe] fn(Peripherals) -> !`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    let inputs = f.sig.inputs;

    quote!(
        #[export_name = "main"]
        pub extern "C" fn main() -> ! {
            let p = unsafe { core::mem::transmute(()) };
            unsafe { __sophgo_rom_rt_macros__main(p) }
        }
        #[allow(non_snake_case)]
        #[inline(always)]
        #(#attrs)*
        #unsafety fn __sophgo_rom_rt_macros__main(#inputs) -> ! {
            #(#stmts)*
        }
    )
    .into()
}
