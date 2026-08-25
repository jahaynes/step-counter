use super::StepCounterService;
use crate::web::api::{AllUsersTotalStepCount, StepCount, UserId};
use chrono::{DateTime, Utc};
use tokio_postgres::{Client, Error, NoTls};

pub struct PostgresStepCounterService {
    client: Client,
}

impl StepCounterService for PostgresStepCounterService {
    async fn record_step_count(
        &self,
        user_id: UserId,
        date_time: DateTime<Utc>,
        step_count: i32,
    ) -> Result<AllUsersTotalStepCount, String> {
        let UserId(uid) = user_id;

        if step_count < 0 {
            return Err("Step count cannot be below 0".to_string());
        }

        let query = r#"
        INSERT INTO step_counter (user_id, date_time, step_count)
        VALUES ($1, $2, $3)
        ON CONFLICT (user_id, date_time)
        DO UPDATE SET step_count = EXCLUDED.step_count
        "#;

        let insertion = self
            .client
            .execute(query, &[&uid, &date_time, &step_count])
            .await
            .map_err(|e| e.to_string())?;

        if insertion == 1 {
            self.get_step_count_all_users(Option::None, Option::None)
                .await
        } else {
            Err("Failed to insert step count".to_string())
        }
    }

    async fn get_step_count(
        &self,
        user_id: UserId,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<StepCount, String> {
        let UserId(uid) = user_id;

        let query = r#"
            SELECT COALESCE(SUM(step_count), 0)::BIGINT AS steps
            FROM step_counter
            WHERE ($1::TIMESTAMPTZ IS NULL OR date_time >= $1)
              AND ($2::TIMESTAMPTZ IS NULL OR date_time <= $2)
              AND user_id = $3
            "#;

        let row = self
            .client
            .query_one(query, &[&start, &end, &uid])
            .await
            .map_err(|e| e.to_string())?;

        let total_steps: i64 = row.try_get("steps").map_err(|e| e.to_string())?;

        Ok(StepCount {
            step_count: total_steps,
        })
    }

    async fn get_step_count_all_users(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<AllUsersTotalStepCount, String> {
        let query = r#"
            SELECT COALESCE(SUM(step_count), 0)::BIGINT AS total_steps
            FROM step_counter
            WHERE ($1::TIMESTAMPTZ IS NULL OR date_time >= $1)
              AND ($2::TIMESTAMPTZ IS NULL OR date_time <= $2)
            "#;

        let row = self
            .client
            .query_one(query, &[&start, &end])
            .await
            .map_err(|e| e.to_string())?;

        let total_steps: i64 = row.try_get("total_steps").map_err(|e| e.to_string())?;

        Ok(AllUsersTotalStepCount {
            all_users_total_step_count: total_steps,
        })
    }

    async fn delete_all_data(&self) -> Result<(), String> {
        let query = r#"
        DELETE FROM step_counter
    "#;

        self.client
            .execute(query, &[])
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

impl PostgresStepCounterService {
    pub async fn new() -> Result<Self, Error> {
        let connection_string = "host=127.0.0.1 port=5432 user=postgres password=mysecretpassword";
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;
        tokio::spawn(async move {
            if let Err(err) = connection.await {
                eprintln!("Postgres connection error: {err}");
            }
        });
        Ok(Self { client })
    }

    pub async fn init(&self) -> Result<(), Error> {
        self.client
            .batch_execute(
                r#"
                    CREATE TABLE IF NOT EXISTS step_counter (
                        user_id UUID NOT NULL,
                        date_time TIMESTAMPTZ NOT NULL,
                        step_count INTEGER NOT NULL,
                        PRIMARY KEY (user_id, date_time)
                    );
                "#,
            )
            .await
    }
}
