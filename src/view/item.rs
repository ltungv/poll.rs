use sailfish::TemplateOnce;
use serde::Serialize;

use crate::model::item::Item;

#[derive(Serialize, TemplateOnce)]
#[template(path = "best_item.stpl")]
pub struct BestItemView<'a> {
    best_item: &'a Option<Item>,
}

impl<'a> BestItemView<'a> {
    pub fn new(best_item: &'a Option<Item>) -> Self {
        Self { best_item }
    }
}
