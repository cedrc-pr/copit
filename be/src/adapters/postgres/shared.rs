use sqlx::{Arguments, AssertSqlSafe, query_as_with};
use uuid::Uuid;
use validator::Validate;

use crate::adapters::postgres::{PostgresAdapter, PostgresAdapterError, messages::Message};

#[derive(serde::Deserialize, Validate)]
pub struct MessagesQuery {
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<u8>,
    pub before: Option<Uuid>,
    pub after: Option<Uuid>,
}

impl PostgresAdapter {
    pub async fn get_room_messages(
        &self,
        messages_query: MessagesQuery,
        room_id: String,
    ) -> Result<Vec<Message>, PostgresAdapterError> {
        let limit = messages_query.limit.unwrap_or(50);
        let mut binds = sqlx::postgres::PgArguments::default();
        binds.add(room_id)?;
        binds.add(chrono::Utc::now())?;

        let condition = match (messages_query.before, messages_query.after) {
            (Some(before), None) => {
                binds.add(before)?;
                "AND messages.id < $3"
            }
            (None, Some(after)) => {
                binds.add(after)?;
                "AND messages.id > $3"
            }
            _ => "",
        };
        binds.add(i32::from(limit))?;

        let get_last = condition.is_empty() || messages_query.before.is_some();

        let sql = format!(
            "
            SELECT *
            FROM (
                SELECT messages.*
                FROM messages
                JOIN rooms ON rooms.id = messages.room_id
                WHERE
                    messages.room_id = $1
                    AND rooms.expires_at > $2
                    {}
                ORDER BY messages.id DESC
                {} {}
            ) AS m
            ORDER BY m.id
            {} {};",
            condition,
            if get_last { "LIMIT" } else { "" },
            match (condition.is_empty(), get_last) {
                (_, false) => "",
                (true, _) => "$3",
                (false, _) => "$4",
            },
            if !get_last { "LIMIT" } else { "" },
            match (condition.is_empty(), get_last) {
                (_, true) => "",
                (true, _) => "$3",
                (false, _) => "$4",
            }
        );

        let messages: Vec<Message> = query_as_with(AssertSqlSafe(sql), binds)
            .fetch_all(&self.pool)
            .await?;

        Ok(messages)

        // let messages: Vec<Message> = query_as(
        //     "
        //     SELECT *
        //     FROM (
        //         SELECT messages.*
        //         FROM messages
        //         LEFT JOIN rooms ON rooms.id = messages.room_id
        //         WHERE
        //           room_id = $1 AND
        //           rooms.expires_at > $2
        //         ORDER BY messages.created_at DESC
        //         LIMIT $3
        //     ) AS m
        //     ORDER BY m.created_at;
        //     ",
        // )
        // .bind(room_id)
        // .bind(chrono::Utc::now())
        // .bind(i64::from(limit))
        // .fetch_all(&self.pool)
        // .await?;

        // Ok(messages)
    }
}
