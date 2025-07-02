use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, DeriveInput, Variant, parse_macro_input};

#[proc_macro_derive(Choice, attributes(desc))]
pub fn derive_choice(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let variants = match &input.data {
        syn::Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("Choice can only be derived for enums"),
    };

    let variant_matches = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let desc = get_description(variant)
            .unwrap_or_else(|| panic!("Missing description for variant {}", variant_name));

        quote! {
            #enum_name::#variant_name => #desc,
        }
    });

    let variants = if let Data::Enum(data_enum) = &input.data {
        data_enum
            .variants
            .iter()
            .map(|v| &v.ident)
            .collect::<Vec<_>>()
    } else {
        panic!("Choice only supports enums")
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
            fn description(&self) -> &'static str {
                match self {
                    #(#variant_matches)*
                }
            }

            pub fn choice(msg: &str) -> ::inquire::error::InquireResult<Self>
            where
                Self: ::std::fmt::Display + ::std::fmt::Debug + Copy + Clone + 'static
            {
                let answer: Self = ::inquire::Select::new(msg, <Self as #module_name::VariantsTrait<Self>>::VARIANTS.to_vec())
                    .prompt()?;

                Ok(answer)
            }
        }

        impl std::fmt::Display for #enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.description())
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_description(variant: &Variant) -> Option<String> {
    variant.attrs.iter().find_map(|attr| {
        if attr.path().is_ident("desc") {
            let mut description = None;
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("text") {
                    let value = meta.value()?;
                    let lit: syn::LitStr = value.parse()?;
                    description = Some(lit.value());
                    Ok(())
                } else {
                    Err(meta.error("expected `text`"))
                }
            })
            .ok()?;
            description
        } else {
            None
        }
    })
}
