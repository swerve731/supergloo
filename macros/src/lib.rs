
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn gloo_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    let method = if attr.is_empty() {
        quote! { get }
    } else {
        let lit = syn::parse_macro_input!(attr as syn::LitStr);
        let m = lit.value().to_lowercase();
        let method = match m.to_string().as_str() {
            "get" => quote! { get },
            "post" => quote! { post },
            "put" => quote! { put },
            "delete" => quote! { delete },
            "patch" => quote! { patch },
            "head" => quote! { head },
            "options" => quote! { options },
            _ => panic!("Invalid HTTP method"),
        };

        method

    };
    
    let input_item = item.clone();
    let input = parse_macro_input!(input_item as ItemFn);
    let fn_ident = &input.sig.ident;

    let fn_name_string = fn_ident.to_string();
        
    let fn_name = syn::LitStr::new(&fn_name_string, fn_ident.span());
    
    
    let expanded = quote! {

        #input


        supergloo::inventory::submit! {
            GlooHandler {
                path: module_path!(),
                    // .replace("::", "/")
                    // .to_string(),
                router: || axum::routing::#method(#fn_ident),
                fn_name: #fn_name,
            }
        }
    };
    expanded.into()
}

