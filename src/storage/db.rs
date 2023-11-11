#![feature(const_generics)]
use bincode::{deserialize, serialize, ErrorKind};
use std::{marker::PhantomData, collections::HashMap, any, sync::Mutex};

use chrono::{DateTime, Utc};    

use super::FileQueueGlobal;




pub struct Column{
    data: *mut u8,
    element_size: u16,
    number_of_element: u32,
}

#[derive(init_table)]
pub struct MyTable {
    // field1: i32,
    // field2: f64,
    // field3: String,
}

// #[proc_macro]
// pub fn init_table(input: TokenStream) -> TokenStream {
//     // Parse the input as a struct
//     let input = parse_macro_input!(input as ItemStruct);
//     let struct_name = &input.ident;

//     // Generate the struct with fields based on the original struct
//     let expanded = quote! {
//         #[derive(Debug)]
//         struct #struct_name {
//             // Add fields based on the original struct fields
//             #input
//         }
//     };

//     // Return the generated code as a TokenStream
//     expanded.into()
// }

// init_table!(Test, {i32, f64, String});