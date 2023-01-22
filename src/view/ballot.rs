use sailfish::TemplateOnce;
use serde::Serialize;
use uuid::Uuid;

use crate::model::item::Item;

use super::item::BestItemView;

#[derive(Serialize, TemplateOnce)]
#[template(path = "ballot.stpl")]
pub struct BallotView<'a> {
    uuid: &'a Uuid,
    best_item_view: BestItemView<'a>,
    ranked_items: &'a [Item],
    unranked_items: &'a [Item],
}

impl<'a> BallotView<'a> {
    pub fn new(
        uuid: &'a Uuid,
        best_item: &'a Option<Item>,
        ranked_items: &'a [Item],
        unranked_items: &'a [Item],
    ) -> Self {
        Self {
            uuid,
            best_item_view: BestItemView::new(best_item),
            ranked_items,
            unranked_items,
        }
    }
}
