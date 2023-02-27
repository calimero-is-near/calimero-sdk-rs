use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, ItemFn, LitStr, ItemImpl, Attribute, Signature, ImplItem};
use syn::{FnArg, Pat, PatType, NestedMeta, Meta, Ident, Fields, ItemStruct};
use syn::parse::Parser;
use syn::parse::ParseBuffer;
use syn::ImplItemMethod;
use syn::spanned::Spanned;
use proc_macro2::Span;
use syn::parse_quote;
use syn::DeriveInput;

#[proc_macro_attribute]
pub fn list_methods(_: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    let fields = if let syn::Data::Struct(struct_data) = ast.data.clone() {
        match struct_data.fields {
            Fields::Named(named_fields) => named_fields.named,
            Fields::Unnamed(unnamed_fields) => unnamed_fields.unnamed,
            Fields::Unit => panic!("list_methods attribute cannot be applied to unit structs"),
        }
    } else {
        panic!("list_methods attribute can only be applied to structs");
    };

    let method_names = fields.iter().map(|f| f.ident.as_ref().unwrap().to_string());

    let gen = quote! {
        #ast

        impl #struct_name {
            pub fn list_all_methods(&self) -> Vec<&'static str> {
                vec![#(#method_names),*]
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn list_methods_impl(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as ItemImpl);

    let struct_name = match ast.self_ty.as_ref() {
        syn::Type::Path(type_path) => type_path.path.get_ident().unwrap(),
        _ => return syn::Error::new_spanned(ast.self_ty, "list_methods attribute can only be applied to named structs").into_compile_error().into(),
    };

    let methods = ast.items.iter().filter_map(|item| {
        match item {
            syn::ImplItem::Method(method) => Some(method.sig.ident.to_string()),
            _ => None,
        }
    });

    let gen = quote! {
        #ast

        impl #struct_name {
            fn list_all_methods(&self) -> Vec<&'static str> {
                vec![#(#methods,)*]
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn list_methods_impl2(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::ItemImpl = syn::parse(item).unwrap();

    let mut methods = Vec::new();
    for item in ast.items.clone() {
        if let syn::ImplItem::Method(mut method) = item.clone() {
            let has_special_attr = method.attrs.iter().any(|attr| attr.path.is_ident("calimero_receive_response"));

            if has_special_attr {
                method.attrs.retain(|attr| !attr.path.is_ident("calimero_receive_response"));
                let method_name = "miki"; //method.sig.ident.to_string();
                let attrs = method.attrs.clone();

                let args: Vec<syn::Pat> = method
                    .sig.clone()
                    .inputs
                    .into_iter()
                    .filter_map(|arg| match arg {
                        syn::FnArg::Receiver(_) => None,
                        syn::FnArg::Typed(syn::PatType { pat, .. }) => Some(*pat),
                    })
                    .collect();

                let last_arg_ident = if let syn::Pat::Ident(x) = args.last().unwrap() {
                    x
                } else {
                    panic!("Could not get ident.")
                };

                //let wanted_arg = method.sig.clone().inputs.clone().last().clone();

                // let inputs = method.sig.inputs.clone();
                // let first_arg = match &inputs[0] {
                //     syn::FnArg::Typed(pat_type) => pat_type.pat.clone(),
                //     _ => unimplemented!(),
                // };

                method.sig.inputs = parse_quote! {
                     //&mut self, #last_arg_ident: Option<Vec<u8>>
                    &mut self, response: Option<Vec<u8>>
                };

                let types: Vec<syn::Type> = method
                    .sig.clone()
                    .inputs
                    .into_iter()
                    .filter_map(|arg| match arg {
                        syn::FnArg::Receiver(_) => None,
                        syn::FnArg::Typed(syn::PatType { ty, .. }) => Some(*ty),
                    })
                    .collect();

                let last_type = types.last();

                let old_block = method.block.clone();

                let expanded = quote! {
                    // #(#attrs)*
                    // fn #method_name(&self) {
                    {
                        println!("Special method!");
                        require!(env::predecessor_account_id().to_string() == CROSS_SHARD_CALL_CONTRACT_ID);
                        //let #wanted_arg =
                        let game_id: Option<usize> =
                        if response.is_none() {
                             None
                        } else {
                            near_sdk::serde_json::from_slice::<Option<usize>>(&response.unwrap()).unwrap()
                        };
                        // if #last_arg_ident.is_none() {
                        //     None
                        // } else {
                        //     near_sdk::serde_json::from_slice::<#last_type>(&#last_arg_ident.unwrap()).unwrap()
                        // };



                        #old_block
                    }
                    //}
                };

                //method.sig.ident = syn::Ident::new(&method_name, method.sig.ident.span());
                method.block = syn::parse2(expanded).unwrap();
                methods.push(syn::ImplItem::Method(method));
            } else {
                let method_name = method.sig.ident.to_string();
                methods.push(item);
            }
        } else {
            methods.push(item);
        }
    }

    let struct_attrs = &ast.attrs;
    let struct_name = &ast.self_ty;
    let gen = quote! {
        //#ast

        #(#struct_attrs)*
        impl #struct_name {
            #(#methods)*
        }
    };
    gen.into()
}

// #[proc_macro_attribute]
// pub fn list_methods_impl2(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let ast: syn::ItemImpl = syn::parse(item).unwrap();
//
//     let mut methods = Vec::new();
//     for item in ast.items {
//         if let syn::ImplItem::Method(method) = item {
//             let mut attrs = method.attrs;
//             let has_special_attr = attrs.iter().any(|attr| attr.path.is_ident("special_method"));
//
//             if has_special_attr {
//                 attrs.retain(|attr| !attr.path.is_ident("special_method"));
//                 let method_name = method.sig.ident.to_string();
//                 let expanded = quote! {
//                     #(#attrs)*
//                     fn #method_name(&self) {
//                         println!("Special method!");
//                     }
//                 };
//                 methods.push(expanded);
//             } else {
//                 let method_name = method.sig.ident.to_string();
//                 methods.push(quote! {
//                     fn #method_name(&self) {}
//                 });
//             }
//         }
//     }
//
//     let struct_name = &ast.self_ty;
//     let gen = quote! {
//         #ast
//
//         impl #struct_name {
//             #(#methods)*
//         }
//     };
//     gen.into()
// }


#[proc_macro_attribute]
pub fn flip_arguments(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_type = parse_macro_input!(item as ItemFn);

    let _types: Vec<syn::Type> = fn_type
        .sig.clone()
        .inputs
        .into_iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(syn::PatType { ty, .. }) => Some(*ty),
        })
        .collect();

    let signature = fn_type.sig;
    let ident = signature.ident;
    let arguments = signature.inputs;
    let return_type = signature.output;

    let block = fn_type.block;

    let attrs = fn_type.attrs;
    let vis = fn_type.vis;

    let first_arg = arguments.first();
    let second_arg = arguments.last();

    TokenStream::from(quote! {
        #(#attrs)* #vis fn #ident ( #second_arg , #first_arg ) #return_type {
            #block
        }
    })
}

#[proc_macro]
pub fn use_calimero(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let generated_code = quote! {
        const CROSS_SHARD_CALL_CONTRACT_ID: &str = #input;
    };
    generated_code.into()
}

// #[proc_macro_attribute]
// pub fn calimero_bindgen(attr: TokenStream, item: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(item as ItemImpl);
//
//     // let item_impl_info = match ItemImplInfo::new(&mut input) {
//     //     Ok(x) => x,
//     //     Err(err) => {
//     //         return err.to_compile_error().into();
//     //     }
//     // };
//
//     for method in &input.items {
//         println!("ovdje sam");
//         println!("{:?}", method);
//         // if method.attr_signature_info.ident == "__contract_abi" {
//         //     return TokenStream::from(
//         //         syn::Error::new_spanned(
//         //             method.attr_signature_info.original_sig.ident.to_token_stream(),
//         //             "use of reserved contract method",
//         //         )
//         //             .to_compile_error(),
//         //     );
//         // }
//     }
//
//     item
// }

#[proc_macro_attribute]
pub fn calimero_response_test(_attr: TokenStream, stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as ItemFn);

    let ItemFn { attrs, vis, sig, block } = input;
    let stmts = &block.stmts;
    let just_for_testing = quote! {
        #(#attrs)* #vis #sig {
            require!(env::predecessor_account_id().to_string() == CROSS_SHARD_CALL_CONTRACT_ID);
            #(#stmts)*
        }
    };
    just_for_testing.into()
}

// modifies fn signature so that the input arg is named response with Option<Vec<u8>> type
// the initial argument is used in generated code and into it the response is deserialized
#[proc_macro_attribute]
pub fn calimero_receive_response1(_: TokenStream, item: TokenStream) -> TokenStream {
    let fn_type = parse_macro_input!(item as ItemFn);

    let types: Vec<syn::Type> = fn_type
        .sig.clone()
        .inputs
        .into_iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(syn::PatType { ty, .. }) => Some(*ty),
        })
        .collect();

    let signature = fn_type.sig;
    let ident = signature.ident;
    let arguments = signature.inputs;
    let args2 = arguments.clone();
    let return_type = signature.output;

    let _args = arguments.iter().map(|fn_arg| match fn_arg {
        FnArg::Typed(PatType { pat, .. }) => match &**pat {
            Pat::Ident(ident) => ident,
            _ => panic!("argument pattern is not a simple ident"),
        }
        FnArg::Receiver(_) => panic!("argument is a receiver"),
    });

    let wanted_type = types.first().unwrap();
    let block = fn_type.block;

    TokenStream::from(quote! {
        fn #ident ( response: Option<Vec<u8>> ) #return_type {
            require!(env::predecessor_account_id().to_string() == CROSS_SHARD_CALL_CONTRACT_ID);
            let #args2 =
            if response.is_none() {
                None
            } else {
                near_sdk::serde_json::from_slice::<#wanted_type>(&response.unwrap()).unwrap()
            };
            #block
        }
    })
}

// Similar as calimero_receive_response1 but leaving example here to see how to extract args and types
// from function signature
#[proc_macro_attribute]
pub fn calimero_receive_response(_: TokenStream, item: TokenStream) -> TokenStream {
    let fn_type = parse_macro_input!(item as ItemFn);
    let attrs = fn_type.attrs;
    let vis = fn_type.vis;
    let signature = fn_type.sig.clone();
    let ident = signature.ident;
    let arguments = signature.inputs;
    let return_type = signature.output;

    let types: Vec<syn::Type> = fn_type
        .sig.clone()
        .inputs
        .into_iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(syn::PatType { ty, .. }) => Some(*ty),
        })
        .collect();

    let wanted_arg = arguments.last().unwrap();

    let last_type = types.last();

    let block = fn_type.block;

    if arguments.len() != 2 {
        panic!("Method using the proc_macro_attribute needs to have two input arguments")
    }

    let first_arg = arguments.first();

    let args: Vec<syn::Pat> = fn_type
        .sig.clone()
        .inputs
        .into_iter()
        .filter_map(|arg| match arg {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(syn::PatType { pat, .. }) => Some(*pat),
        })
        .collect();

    let last_arg_ident = if let syn::Pat::Ident(x) = args.last().unwrap() {
        x
    } else {
        panic!("Could not get ident.")
    };

    TokenStream::from(quote! {
        #(#attrs)* #vis fn #ident ( #first_arg, #last_arg_ident: Option<Vec<u8>> ) #return_type {
            require!(env::predecessor_account_id().to_string() == CROSS_SHARD_CALL_CONTRACT_ID);
            let #wanted_arg =
            if #last_arg_ident.is_none() {
                None
            } else {
                near_sdk::serde_json::from_slice::<#last_type>(&#last_arg_ident.unwrap()).unwrap()
            };

            #block
        }
    })
}