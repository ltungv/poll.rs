use actix_web_flash_messages::IncomingFlashMessages;
use sailfish::TemplateOnce;
use serde::Serialize;

use crate::{model::item::Item, view::component::BestItemView};

use super::component::FlashMessagesView;

#[derive(Serialize, TemplateOnce)]
#[template(path = "index.stpl")]
pub struct IndexView<'a> {
    best_item_view: BestItemView<'a>,
    flash_messages_view: FlashMessagesView<'a>,
}

impl<'a> IndexView<'a> {
    pub fn new(best_item: &'a Option<Item>, flashes: &'a IncomingFlashMessages) -> Self {
        IndexView {
            best_item_view: BestItemView::new(best_item),
            flash_messages_view: FlashMessagesView::new(flashes),
        }
    }
}
