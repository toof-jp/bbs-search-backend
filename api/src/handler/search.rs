use axum::extract::{Query, State};
use axum::Json;
use kernel::model::{Count, Res};

use crate::model::search::SearchRequest;

pub async fn search_handler(
    state: State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Json<Vec<Res>> {
    Json(get_res(&state.pool, params.into()).await)
}

pub async fn search_count_handler(
    state: State<AppState>,
    Query(params): Query<SearchRequest>,
) -> Json<Count> {
    Json(get_res_count(&state.pool, params.into()).await)
}
