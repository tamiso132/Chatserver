use chrono::{Datelike, Timelike, Utc};
use serde_derive::{Deserialize, Serialize};

use super::id::*;

#[derive(Deserialize, Serialize)]
pub struct Receipt {
    product: Vec<Order>,
    timestamp: Date,
}

#[derive(Deserialize, Serialize)]
pub struct Order {
    product_id: ProductId,
    product_amount: u8,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Product {
    pub(crate) name: [u8; 25],
    pub(crate) category_id: CategoryId,
    pub(crate) brand_id: BrandId,
}

pub struct CompactProduct {
    pub(crate) name: [u8; 25],
    pub(crate) category_id: CategoryId,
    pub(crate) brand_id: BrandId,
}

impl From<Product> for CompactProduct {
    fn from(value: Product) -> Self {
        Self {
            name: value.name,
            category_id: value.category_id,
            brand_id: value.brand_id,
        }
    }
}

impl Default for Product {
    fn default() -> Self {
        Self {
            name: Default::default(),
            category_id: Default::default(),
            brand_id: Default::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Category {
    category_name: String,
}

impl Category {
    pub fn new(name: String) -> Self {
        Self {
            category_name: name,
        }
    }
}
impl Default for Category {
    fn default() -> Self {
        Self {
            category_name: String::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Brand {
    brand_name: String,
}

impl Brand {
    pub fn new(name: String) -> Self {
        Self { brand_name: name }
    }
}
impl Default for Brand {
    fn default() -> Self {
        Self {
            brand_name: String::default(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Date {
    year: u16,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Date {
    pub fn new() -> Self {
        let x = Utc::now();
        Self {
            year: x.year() as u16,
            day: x.day() as u8,
            hour: x.hour() as u8,
            minute: x.minute() as u8,
            second: x.second() as u8,
        }
    }
}
