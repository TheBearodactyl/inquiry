use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, parse_macro_input};

fn enum_choice_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let enum_name = &input.ident;

    let variants = if let Data::Enum(data_enum) = &input.data {
        data_enum
            .variants
            .iter()
            .map(|v| &v.ident)
            .collect::<Vec<_>>()
    } else {
        panic!("Choice only support enums")
    };

    let module_name = format_ident!("__enum_choice_for_{}", enum_name.to_string().to_lowercase());

    let expanded = quote! {
            mod #module_name {
                use super::*;

                #[doc(hidden)]
                pub trait Variants<T: 'static> {
                    const VARIANTS: &'static [T];
                }

                impl Variants<#enum_name> for #enum_name {
                    const VARIANTS: &'static [#enum_name] = &[#(#enum_name::#variants),*];
                }

                pub use Variants as VariantsTrait;
            }

            impl #enum_name {
		        pub fn choice(msg: &str) -> ::inquire::error::InquireResult<Self>
                where
                    Self: ::std::fmt::Display + ::std::fmt::Debug + Copy + Clone + 'static,
                {

                    let answer: Self = ::inquire::Select::new(msg, <Self as #module_name::VariantsTrait<Self>>::VARIANTS.to_vec())
                    .prompt()?;

                    Ok(answer)
                }
            }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Choice)]
pub fn derive_enum_choice(input: TokenStream) -> TokenStream {
    enum_choice_impl(input)
}