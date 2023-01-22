use actix_web_flash_messages::IncomingFlashMessages;
use sailfish::TemplateOnce;
use serde::Serialize;
use uuid::Uuid;

use crate::model::item::Item;

use super::component::{BestItemView, FlashMessagesView};

#[derive(Serialize, TemplateOnce)]
#[template(path = "ballot.stpl")]
pub struct BallotView<'a> {
    uuid: &'a Uuid,
    best_item_view: BestItemView<'a>,
    flash_messages_view: FlashMessagesView<'a>,
    ranked_items: &'a [Item],
    unranked_items: &'a [Item],
}

impl<'a> BallotView<'a> {
    pub fn new(
        uuid: &'a Uuid,
        best_item: &'a Option<Item>,
        flashes: &'a IncomingFlashMessages,
        ranked_items: &'a [Item],
        unranked_items: &'a [Item],
    ) -> Self {
        Self {
            uuid,
            best_item_view: BestItemView::new(best_item),
            flash_messages_view: FlashMessagesView::new(flashes),
            ranked_items,
            unranked_items,
        }
    }
}
