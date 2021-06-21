use sqlx::SqlitePool;

use crate::{
    irv::{run_instant_runoff_voting, PollResult},
    models::Item,
};

pub(crate) async fn get_poll_result(pool: &SqlitePool) -> crate::Result<Option<Item>> {
    // Query for items sorted by user id and vote order
    let records = query!(
        r#"
        SELECT 
            rankings.ballot_id as ballot_id,
            items.id as item_id,
            items.title as item_title,
            items.content as item_content,
            items.done as item_done
        FROM rankings INNER JOIN items ON rankings.item_id = items.id
        WHERE NOT items.done 
        ORDER BY rankings.ballot_id ASC, rankings.ord ASC;
        "#
    )
    .fetch_all(pool)
    .await?;

    // Group votes by user id
    let mut votes = Vec::new();
    let mut current_ballot_votes = Vec::new();
    let mut last_ballot_id = None;
    for record in records {
        if let Some(n) = last_ballot_id {
            if n != record.ballot_id {
                votes.push(current_ballot_votes);
                current_ballot_votes = Vec::new();
            }
        }
        last_ballot_id = Some(record.ballot_id);
        current_ballot_votes.push(Item {
            id: record.item_id,
            title: record.item_title,
            content: record.item_content,
            done: record.item_done,
        });
    }
    if last_ballot_id.is_some() {
        votes.push(current_ballot_votes);
    }

    // Get poll result
    let votes: Vec<_> = votes.iter().map(|v| v.as_slice()).collect();
    let best_item = match run_instant_runoff_voting(&votes) {
        PollResult::NoWinner => None,
        PollResult::Tied(winners) => Some(winners[0].clone()),
        PollResult::Winner(winner) => Some(winner.clone()),
    };
    Ok(best_item)
}

pub(crate) async fn get_all_undone_items(pool: &SqlitePool) -> crate::Result<Vec<Item>> {
    // Query for items sorted by user id and vote order
    let items = query_as!(Item, r#"SELECT  * FROM items WHERE NOT items.done"#)
        .fetch_all(pool)
        .await?;
    Ok(items)
}
