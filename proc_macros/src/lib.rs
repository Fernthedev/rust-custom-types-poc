extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, ToTokens};

use syn::{parse_macro_input, Error, LitStr, Token};
use syn::{punctuated::Punctuated, ItemStruct};

#[proc_macro_attribute]
pub fn custom_type_class(attr: TokenStream, item: TokenStream) -> TokenStream {
    let punctuated_args =
        parse_macro_input!(attr with Punctuated<LitStr, Token![,]>::parse_separated_nonempty);
    let input = parse_macro_input!(item as ItemStruct);

    // Build the trait implementation
    match impl_custom_type_class(punctuated_args, input) {
        Ok(ts) => ts,
        Err(err) => err.to_compile_error().into(),
    }
}

fn impl_custom_type_class(
    punctuated_args: Punctuated<LitStr, Token![,]>,
    input: ItemStruct,
) -> Result<TokenStream, Error> {
    let args: Vec<String> = punctuated_args.iter().map(LitStr::value).collect();

    // TODO: Class is redundant, should we use it or use the struct name?
    // TODO: Get the namespace from the mod of the struct?
    // TODO: Make override_class an actual type instead of a string so compiler can see if it exists. Possibly support strings in case of a stripped class?
    let (mut namespace, class, override_class) = match args.as_slice() {
        [n, c, o] => (n, c, o),
        _ => {
            let msg = format!("Expected 3 arguments, found {}", args.len());
            return Err(Error::new_spanned(punctuated_args, msg));
        }
    };

    let override_class_ident = syn::Ident::new(override_class, Span::call_site());

    let input_clone = input.clone();

    let name = input.ident;

    let gen = quote! {
        #input_clone

        impl CustomTypeClassTrait for #name {
            fn install() {
                println!("Installing custom type {}::{}!", #namespace, stringify!(#name));
                // TODO: Install
            }
        }

        impl Il2CppObject for #name {
            const klass: Il2CppClass = Il2CppClass {
                namespace: #namespace,
                name: stringify!(#name),
            };
        }


        // How to make only if override_class is specified?
        impl #override_class_ident for #name {

        }
    };
    Ok(gen.into())
}
