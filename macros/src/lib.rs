
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn gloo_handler(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse attributes (e.g., HTTP method) or default to GET
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
    
    // Parse the function
    let input_item = item.clone();
    let input = parse_macro_input!(input_item as ItemFn);
    let fn_ident = &input.sig.ident;

    let fn_name_string = fn_ident.to_string();
        
    let fn_name = syn::LitStr::new(&fn_name_string, fn_ident.span());
    
    
    // Generate the expanded code
    let expanded = quote! {

        #input


        inventory::submit! {
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


#[proc_macro]
pub fn gloo_routes(_input: TokenStream) -> TokenStream {

    let expanded = quote! {
        {
            let mut router = Router::new();
            for handler in inventory::iter::<GlooHandler> {


                let mut path = handler.path.split("::").collect::<Vec<_>>();

                // if !path.contains("routes") {
                //     panic!("Path must contain 'routes'");
                // }

                for i in 0..path.len() {
                    if path[i] == "routes" {
                        path.remove(i);
                        break;
                    } else {
                        path[i] = ""
                    }
                }


                let mut path = path.join("/").replace("::", "/");

                if handler.fn_name != "base" {
                    path = format!("{}/{}", path, handler.fn_name);
                }

                if !path.starts_with("/") {
                    path = format!("/{}", path);
                }


                dbg!(&path);


                router = router.route(&path, (handler.router)());
            }
            router
        }
    };
    expanded.into()
}