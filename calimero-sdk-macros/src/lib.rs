use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse_macro_input, ItemFn, LitStr};
use syn::{FnArg, Pat, PatType};

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