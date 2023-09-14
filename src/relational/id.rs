use serde_derive::{Deserialize, Serialize};

macro_rules! create_id {
    ($name:ident) => {
        #[derive(Deserialize, Serialize)]
        pub struct $name {
            pub(crate) ver: u16,
            pub(crate) id: u16,
        }
        impl Default for $name {
            fn default() -> Self {
                Self {
                    ver:Default::default(),
                    id: Default::default(),
                }
            }
        }
    };
}

create_id!(ProductId);
create_id!(BrandId);
create_id!(CategoryId);
