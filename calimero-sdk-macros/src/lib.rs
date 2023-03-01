use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input, FnArg, parse_quote};

const CALIMERO_RECEIVE_RESPONSE_ATTRIBUTE: &str = "calimero_receive_response";

/// Modifies fn signature under impl for any fn denoted with #[calimero_receive_response]
/// so that the last input arg of such function is renamed to `response` with Option<Vec<u8>> as
/// type, the argument initially provided by the developer is used in generated code and into
/// it - the response is deserialized. If the developer used optional type, error can be handled
/// in the contract method, otherwise the method will panic if `response` is None
#[proc_macro_attribute]
pub fn calimero_expand(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: syn::ItemImpl = syn::parse(item).unwrap();

    let mut methods = Vec::new();
    for item in ast.items.clone() {
        if let syn::ImplItem::Method(mut method) = item.clone() {
            let has_special_attr = method.attrs.iter().any(|attr| attr.path.is_ident(CALIMERO_RECEIVE_RESPONSE_ATTRIBUTE));

            if has_special_attr {
                method.attrs.retain(|attr| !attr.path.is_ident(CALIMERO_RECEIVE_RESPONSE_ATTRIBUTE));

                if method.sig.inputs.len() != 2 {
                    panic!("Method using the proc_macro_attribute #[calimero_receive_response] needs to have exactly two input arguments")
                }

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

                let types: Vec<syn::Type> = method
                    .sig.clone()
                    .inputs
                    .into_iter()
                    .filter_map(|arg| match arg {
                        syn::FnArg::Receiver(_) => None,
                        syn::FnArg::Typed(syn::PatType { ty, .. }) => Some(*ty),
                    })
                    .collect();

                let last_type_arg = types.last();
                let input_arg_is_optional = is_optional(method.sig.inputs.last().unwrap());

                let first_argument = method.sig.inputs.first().unwrap();
                method.sig.inputs = parse_quote! {
                    #first_argument, response: Option<Vec<u8>>
                };

                let old_block = method.block.clone();

                let expanded =
                    if input_arg_is_optional {
                        quote! {
                            {
                                require!(env::predecessor_account_id().to_string() == CROSS_SHARD_CALL_CONTRACT_ID);
                                let #last_arg_ident: #last_type_arg =
                                if response.is_none() {
                                    None
                                } else {
                                    near_sdk::serde_json::from_slice::<#last_type_arg>(&response.unwrap()).unwrap()
                                };
                                #old_block
                            }
                        }
                    } else {
                        quote! {
                            {
                                require!(env::predecessor_account_id().to_string() == CROSS_SHARD_CALL_CONTRACT_ID);
                                if response.is_none() {
                                    panic!("Expected cross shard call response, but received None");
                                }
                                let #last_arg_ident: #last_type_arg =
                                    near_sdk::serde_json::from_slice::<#last_type_arg>(&response.unwrap()).unwrap();
                                #old_block
                            }
                        }
                    };

                method.block = syn::parse2(expanded).unwrap();
                methods.push(syn::ImplItem::Method(method));
            } else {
                methods.push(item);
            }
        } else {
            methods.push(item);
        }
    }

    let struct_attrs = &ast.attrs;
    let struct_name = &ast.self_ty;
    let gen = quote! {
        #(#struct_attrs)*
        impl #struct_name {
            #(#methods)*
        }
    };
    gen.into()
}

fn is_optional(arg: &FnArg) -> bool {
    if let FnArg::Typed(typed) = arg {
        if let syn::Type::Path(path) = &*typed.ty {
            if let Some(segment) = path.path.segments.last() {
                if segment.ident.to_string() == "Option" {
                    return true;
                }
            }
        }
    }
    false
}

/// This macro needs to be defined before making a cross_call, or receiving
/// a cross_call_receive_response
///
/// E.g. before the definition of the contract
/// calimero_cross_shard_connector!("xsc_connector.shard_id.dev.calimero.testnet");
#[proc_macro]
pub fn calimero_cross_shard_connector(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);

    let generated_code = quote! {
        const CROSS_SHARD_CALL_CONTRACT_ID: &str = #input;
    };
    generated_code.into()
}
