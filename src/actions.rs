use sqlx::SqlitePool;

use crate::{
    irv::{run_instant_runoff_voting, PollResult},
    models::Item,
};

pub(crate) async fn get_poll_result(pool: &SqlitePool) -> crate::Result<Option<Item>> {
    // Query for items sorted by ballot id and ranking order
    let records = query!(
        r#"
        SELECT 
            rankings.ballot_id,
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

    let mut ballots = vec![Vec::new()];
    let mut current_ballot = 0;
    let mut last_ballot_id = None;
    for record in records {
        if let Some(ballot_id) = last_ballot_id {
            if ballot_id != record.ballot_id {
                ballots.push(Vec::new());
                current_ballot += 1;
            }
        }
        last_ballot_id = Some(record.ballot_id);
        ballots[current_ballot].push(Item {
            id: record.item_id,
            title: record.item_title,
            content: record.item_content,
            done: record.item_done,
        });
    }

    // Get poll result
    let ballots: Vec<_> = ballots.iter().map(|v| v.as_slice()).collect();
    let best_item = match run_instant_runoff_voting(&ballots) {
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

pub(crate) async fn get_ballot_items_status(
    pool: &SqlitePool,
    uuid: String,
) -> crate::Result<(Vec<Item>, Vec<Item>)> {
    let (ranked_items, unranked_items) = join!(
        query_as!(
            Item,
            r#"
            SELECT items.id, items.title, items.content, items.done
            FROM rankings 
                INNER JOIN items ON rankings.item_id = items.id
                INNER JOIN ballots ON rankings.ballot_id = ballots.id
            WHERE NOT items.done AND ballots.uuid = ?
            ORDER BY rankings.ord ASC;
        "#,
            uuid
        )
        .fetch_all(pool),
        query_as!(
            Item,
            r#"
            SELECT items.id, items.title, items.content, items.done
            FROM items 
            WHERE items.id NOT IN (
                SELECT item_id FROM rankings INNER JOIN ballots WHERE ballots.uuid = ?
            )
        "#,
            uuid
        )
        .fetch_all(pool)
    );
    Ok((ranked_items?, unranked_items?))
}
