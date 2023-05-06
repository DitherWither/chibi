use rand::{distributions::Alphanumeric, Rng};
use sqlx::{PgPool, Row};

#[derive(Debug, thiserror::Error)]
pub(crate) enum ShortenError {
    #[error("Invalid url, check that the url is valid and try again.")]
    InvalidUrl,
    #[error("Unknown database error: {0}")]
    DatabaseError(sqlx::Error),
}

#[derive(Clone)]
pub(crate) struct ShortenerService {
    db: PgPool,
}

impl ShortenerService {
    pub(crate) fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// The database query to get the url from the database.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the url.
    /// * `db` - The database connection.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(url))` - The url was found in the database.
    /// * `Ok(None)` - The url was not found in the database.
    /// * `Err(e)` - There was an error querying the database.
    pub(crate) async fn redirect(&self, id: &str) -> Result<Option<String>, sqlx::Error> {
        let url = sqlx::query("SELECT * FROM urls WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db)
            .await;

        match url {
            Ok(Some(row)) => Ok(Some(row.get("url"))),
            Ok(None) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub(crate) async fn shorten(&self, url: &str) -> Result<String, ShortenError> {
        // Validate the url.

        if let Err(_e) = url::Url::parse(url) {
            return Err(ShortenError::InvalidUrl);
        }

        // Check if the url is already in the database.
        let id_row = sqlx::query("SELECT * FROM urls WHERE url = $1")
            .bind(url)
            .fetch_optional(&self.db)
            .await
            .map_err(ShortenError::DatabaseError)?;

        // If the url is already in the database, return the id.
        if let Some(row) = id_row {
            return Ok(row.get("id"));
        }

        // Otherwise, generate a new id.
        let id: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        match sqlx::query("INSERT INTO urls (id, url) VALUES ($1, $2)")
            .bind(&id)
            .bind(url)
            .execute(&self.db)
            .await
        {
            Ok(_) => Ok(id),
            Err(e) => Err(ShortenError::DatabaseError(e)),
        }
    }
}
