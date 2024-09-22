use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{repository::Repositories, search::models::SearchQuery};
use resource::repository::RealInfrastructureRepository;
use serde_json::json;
use usecase::search::SearchUseCase;

pub async fn full_search(
    State(repository): State<RealInfrastructureRepository>,
    Query(search_query): Query<SearchQuery>,
) -> impl IntoResponse {
    let search_use_case = SearchUseCase {
        repository: repository.search_repository(),
    };

    match search_query {
        SearchQuery { query: None } => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "reason": "query is required" })),
        )
            .into_response(),
        SearchQuery { query: Some(query) } => match search_use_case.full_search(query).await {
            Ok(result) => (StatusCode::OK, Json(result)).into_response(),
            Err(err) => {
                tracing::error!("{}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "reason": "unknown error" })),
                )
                    .into_response()
            }
        },
    }
}