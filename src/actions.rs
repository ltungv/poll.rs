use sqlx::{query, SqlitePool};

use crate::{
    irv::{run_instant_runoff_voting, PollResult},
    models,
};

pub(crate) async fn get_poll_result(pool: &SqlitePool) -> crate::Result<Option<models::Item>> {
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
        ORDER BY rankings.ballot_id ASC, rankings.ord ASC
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
        ballots[current_ballot].push(models::Item {
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

pub(crate) async fn get_ballot_id(pool: &SqlitePool, uuid: &str) -> crate::Result<Option<i64>> {
    let record = query!("SELECT id FROM ballots WHERE ballots.uuid = ?", uuid)
        .fetch_optional(pool)
        .await?;
    Ok(record.map(|r| r.id))
}

pub(crate) async fn get_ballot_rankings(
    pool: &SqlitePool,
    ballot_id: i64,
) -> crate::Result<(Vec<models::Item>, Vec<models::Item>)> {
    let (ranked_items, unranked_items) = try_join!(
        query_as!(
            models::Item,
            r#"
            SELECT items.id, items.title, items.content, items.done
            FROM rankings 
                INNER JOIN items ON rankings.item_id = items.id
            WHERE NOT items.done AND rankings.ballot_id = ?
            ORDER BY rankings.ord ASC;
            "#,
            ballot_id
        )
        .fetch_all(pool),
        query_as!(
            models::Item,
            r#"
            SELECT items.id, items.title, items.content, items.done
            FROM items 
            WHERE items.id NOT IN (
                SELECT item_id FROM rankings WHERE rankings.ballot_id = ?
            )
            "#,
            ballot_id
        )
        .fetch_all(pool)
    )?;
    Ok((ranked_items, unranked_items))
}

pub(crate) async fn new_ballot(pool: &SqlitePool, uuid: &str) -> crate::Result<()> {
    query!(
        "INSERT INTO ballots(uuid) VALUES (?) ON CONFLICT (uuid) DO NOTHING",
        uuid
    )
    .execute(pool)
    .await?;
    Ok(())
}


pub(crate) async fn new_ballot_rankings(
    pool: &SqlitePool,
    ballot_id: i64,
    ranked_item_ids: &[i64],
) -> crate::Result<()> {
    // Since there's no bulk insert option, we are building the query by appending strings
    let mut insert_query = String::from("INSERT INTO rankings(ballot_id, item_id, ord) VALUES");
    for (ord, item_id) in ranked_item_ids.iter().enumerate() {
        if ord != 0 {
            insert_query += ",";
        }
        insert_query += format!("({}, {}, {})", ballot_id, item_id, ord).as_str();
    }
    insert_query += ";";

    let mut tx = pool.begin().await?;
    // Remove all rankings
    query!(
        r#"
        DELETE FROM rankings
        WHERE rankings.ballot_id 
        IN (SELECT id FROM ballots WHERE ballots.id = ?)
        "#,
        ballot_id,
    )
    .execute(&mut tx)
    .await?;
    // Insert new rankings
    if !ranked_item_ids.is_empty() {
        query(&insert_query).execute(&mut tx).await?;
    }
    tx.commit().await?;

    Ok(())
}
