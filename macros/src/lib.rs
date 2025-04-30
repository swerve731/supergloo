
use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, FnArg, ItemFn};

#[proc_macro_attribute]
pub fn gloo_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.is_empty() {
        panic!("Missing Gloo method kind")
    };

    let attr_lit = syn::parse_macro_input!(attr as syn::LitStr);
    
    let attr_string = attr_lit.value().to_lowercase();
    let method = match attr_string.to_string().as_str() {
        "get" => quote! { get },
        "post" => quote! { post },
        "put" => quote! { put },
        "delete" => quote! { delete },
        "patch" => quote! { patch },
        "head" => quote! { head },
        "options" => quote! { options },
        "view" => quote! { get },
        _ => panic!("Invalid Gloo method kind"),
    };

    
    
    let input_item = item.clone();
    let input = parse_macro_input!(input_item as ItemFn);
    let fn_ident = &input.sig.ident;

    let fn_name_string = fn_ident.to_string();
    let fn_name = syn::LitStr::new(&fn_name_string, fn_ident.span());
    
    // view kinds get layout from the layout.rs file in the mod
    
    let kind = match attr_string.to_string().as_str() {
        "view" => quote! { supergloo::GlooHandlerKind::View(|| supergloo::utils::box_view_fn(#fn_ident())) },
        _ => quote! { supergloo::GlooHandlerKind::Route },
    };



    let expanded = quote! {
        #input

        supergloo::inventory::submit! {
            GlooHandler {
                path: module_path!(),
                    // .replace("::", "/")
                    // .to_string(),
                router: || axum::routing::#method(#fn_ident),
                fn_name: #fn_name,
                kind: #kind,
            }
        }
    };
    expanded.into()
}


#[proc_macro_attribute]
pub fn gloo_layout(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    let child_param: FnArg = parse_quote!(child: supergloo::maud::Markup);

    // 3. Check if the parameter already exists (optional, but good practice)
    let has_child_param = input_fn.sig.inputs.iter().any(|arg| {
        if let FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                return pat_ident.ident == "child";
            }
        }
        false
    });

    let fn_ident = &input_fn.sig.ident;
    
    // Add the parameter if it doesn't exist
    if !has_child_param {
        input_fn.sig.inputs.push(child_param);
    } else {
        // later will Issue a warning or do nothing if it already exists
        // syn::Error::new_spanned(input_fn.sig.inputs, "Function already has a 'child' parameter.")
        //     .to_compile_error() 
    }   
    
    let expanded = quote! {

        #input_fn // This now includes the modified parameters

        supergloo::inventory::submit! {
            GlooLayout {
                path: module_path!(),
                layout_fn: #fn_ident,
            }
        }
    };
    expanded.into()
} 