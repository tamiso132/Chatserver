use serde_derive::{Deserialize, Serialize};

macro_rules! create_id {
    ($name:ident) => {
        #[derive(Deserialize, Serialize)]
        pub struct $name {
            pub(crate) id: u16,
            pub(crate) version: u8,
        }
        impl Default for $name {
            fn default() -> Self {
                Self {
                    id: Default::default(),
                    version: Default::default(),
                }
            }
        }
    };
}

create_id!(ProductId);
create_id!(BrandId);
create_id!(CategoryId);
