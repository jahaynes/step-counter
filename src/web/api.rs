use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserId(pub Uuid);

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct StepCountRecord {
    pub user_id: UserId,
    pub date_time: DateTime<Utc>,
    pub step_count: i32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UserDateRange {
    pub user_id: Uuid,
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DateRange {
    pub start: Option<DateTime<Utc>>,
    pub end: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepCount {
    pub step_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllUsersTotalStepCount {
    pub all_users_total_step_count: i64,
}
