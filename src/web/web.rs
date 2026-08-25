use crate::{
    service::{StepCounterService, pg::PostgresStepCounterService},
    web::{
        AppState,
        api::{
            AllUsersTotalStepCount, DateRange, StepCount, StepCountRecord, UserDateRange, UserId,
        },
    },
};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{delete, get, post},
};
use std::{io::Error, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    record_step_count,
    all_users_step_count,
    user_step_count,
    delete_all_data
))]
struct ApiDoc;

pub async fn serve(svc: PostgresStepCounterService) -> Result<(), Error> {
    let app_state = AppState {
        step_counter: Arc::new(svc),
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    let app = Router::new()
        .route("/steps/record", post(record_step_count))
        .route("/steps/all-users-step-count", get(all_users_step_count))
        .route("/steps/user-step-count", get(user_step_count))
        .route("/steps/delete_all", delete(delete_all_data))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(app_state);

    axum::serve(listener, app).await
}

#[utoipa::path(post, path = "/steps/record")]
async fn record_step_count<S>(
    State(app): State<AppState<S>>,
    Json(record): Json<StepCountRecord>,
) -> Result<Json<AllUsersTotalStepCount>, String>
where
    S: StepCounterService + Send + Sync + 'static,
{
    let total = app
        .step_counter
        .record_step_count(record.user_id, record.date_time, record.step_count)
        .await?;
    Ok(Json(total))
}

#[utoipa::path(
    get,
    path = "/steps/all-users-step-count",
    params(
        ("start" = Option<DateTime<Utc>>, Query, description = "Start of date range"),
        ("end" = Option<DateTime<Utc>>, Query, description = "End of date range"),
    ),)]
async fn all_users_step_count<S>(
    State(app): State<AppState<S>>,
    Query(params): Query<DateRange>,
) -> Result<Json<AllUsersTotalStepCount>, String>
where
    S: StepCounterService + Send + Sync + 'static,
{
    let total = app
        .step_counter
        .get_step_count_all_users(params.start, params.end)
        .await?;
    Ok(Json(total))
}

#[utoipa::path(
    get,
    path = "/steps/user-step-count",
    params(
        ("user_id" = Uuid, Query, description = "User ID"),
        ("start" = Option<DateTime<Utc>>, Query, description = "Start of date range"),
        ("end" = Option<DateTime<Utc>>, Query, description = "End of date range"),
    ),)]
async fn user_step_count<S>(
    State(app): State<AppState<S>>,
    Query(params): Query<UserDateRange>,
) -> Result<Json<StepCount>, String>
where
    S: StepCounterService + Send + Sync + 'static,
{
    let total = app
        .step_counter
        .get_step_count(UserId(params.user_id), params.start, params.end)
        .await?;

    Ok(Json(total))
}

#[utoipa::path(
    delete,
    path = "/steps/delete_all",
    responses(
        (status = 204, description = "All database data deleted"),
    ),
)]
async fn delete_all_data<S>(State(app): State<AppState<S>>) -> Result<(), String>
where
    S: StepCounterService + Send + Sync + 'static,
{
    app.step_counter.delete_all_data().await?;
    Ok(())
}
