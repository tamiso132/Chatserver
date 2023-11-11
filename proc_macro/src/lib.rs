
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields};

#[proc_macro]
pub fn init_table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let add_record_function = generate_add_record_function(&input);

    let expanded = quote! {
        #input

        #add_record_function
    };

    expanded.into()
}

fn generate_add_record_function(input: &ItemStruct) -> proc_macro2::TokenStream {
    let struct_name = &input.ident;
    let add_record_params = generate_add_record_params(&input.fields);

    quote! {
        impl #struct_name {
            pub fn add_record(#(#add_record_params),*) {
                // Implement the logic to add a record based on the field values
                // For demonstration purposes, print the values to the console
                println!("Adding record with values: {:?}", (#(#add_record_params),*));
            }
        }
    }
}

fn generate_add_record_params(fields: &Fields) -> Vec<proc_macro2::TokenStream> {
    match fields {
        Fields::Named(named_fields) => {
            named_fields
                .named
                .iter()
                .map(|field| {
                    let field_name = field.ident.as_ref().expect("Field has no identifier");
                    let field_type = &field.ty;
                    quote! { #field_name: #field_type }
                })
                .collect()
        }
        _ => {
            panic!("Only named fields are supported for this example");
        }
    }
}
