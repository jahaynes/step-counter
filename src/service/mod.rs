use chrono::{DateTime, Utc};

use crate::web::api::{AllUsersTotalStepCount, StepCount, UserId};

pub trait StepCounterService {
    async fn record_step_count(
        &self,
        user_id: UserId,
        date_time: DateTime<Utc>,
        step_count: i32,
    ) -> Result<AllUsersTotalStepCount, String>;

    async fn get_step_count(
        &self,
        user_id: UserId,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<StepCount, String>;

    async fn get_step_count_all_users(
        &self,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
    ) -> Result<AllUsersTotalStepCount, String>;

    async fn delete_all_data(&self) -> Result<(), String>;
}

pub mod pg;
