use sailfish::TemplateOnce;
use serde::Serialize;

use crate::{model::item::Item, view::item::BestItemView};

#[derive(Serialize, TemplateOnce)]
#[template(path = "index.stpl")]
pub struct IndexView<'a> {
    best_item_view: BestItemView<'a>,
}

impl<'a> IndexView<'a> {
    pub fn new(best_item: &'a Option<Item>) -> Self {
        IndexView {
            best_item_view: BestItemView::new(best_item),
        }
    }
}
