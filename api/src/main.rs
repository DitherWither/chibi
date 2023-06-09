mod routes;
mod services;

use crate::services::shortener;

use std::path::PathBuf;

use anyhow::Result;
use poem::{endpoint::StaticFilesEndpoint, middleware::AddData, EndpointExt, Route};
use poem_openapi::OpenApiService;
use shuttle_poem::ShuttlePoem;
use shuttle_service::CustomError;
use sqlx::{Executor, PgPool};

/// Initialize the database.
///
/// This will be called during server startup.
///
/// It runs the SQL schema in `schema.sql`.
async fn init_db(pool: &PgPool) -> Result<()> {
    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    Ok(())
}

#[shuttle_runtime::main]
async fn poem(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder] static_folder: PathBuf,
) -> ShuttlePoem<impl poem::Endpoint> {
    init_db(&pool).await?;

    let static_files: StaticFilesEndpoint =
        StaticFilesEndpoint::new(static_folder).index_file("index.html");

    let api_service =
        OpenApiService::new(routes::ShortnerApi, "Shortener API", "0.1.0").server("/u");
    let ui = api_service.swagger_ui();

    let shortener_service = shortener::ShortenerService::new(pool.clone());

    let app = Route::new()
        // Static files
        .nest("/", static_files)
        // API
        .nest("/u", api_service)
        .nest("/u/docs", ui)
        // Database
        .with(AddData::new(shortener_service));

    Ok(app.into())
}
