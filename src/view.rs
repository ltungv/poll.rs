use actix_web_flash_messages::IncomingFlashMessages;
use sailfish::TemplateOnce;
use serde::Serialize;
use uuid::Uuid;

use crate::model::Item;

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

#[derive(Serialize, TemplateOnce)]
#[template(path = "flash_messages.stpl")]
pub struct FlashMessagesView<'a> {
    flashes: &'a IncomingFlashMessages,
}

impl<'a> FlashMessagesView<'a> {
    pub fn new(flashes: &'a IncomingFlashMessages) -> Self {
        Self { flashes }
    }

    pub fn notification_class(level: actix_web_flash_messages::Level) -> String {
        let classes = [
            "notification",
            "m-1",
            "p-2",
            match level {
                actix_web_flash_messages::Level::Debug => "is-link",
                actix_web_flash_messages::Level::Info => "is-info",
                actix_web_flash_messages::Level::Success => "is-success",
                actix_web_flash_messages::Level::Warning => "is-warning",
                actix_web_flash_messages::Level::Error => "is-danger",
            },
        ];
        classes.join(" ")
    }
}
