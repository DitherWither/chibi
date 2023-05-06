use poem::web::Data;
use poem_openapi::{
    param::Path,
    payload::{Form, PlainText},
    ApiResponse, Object, OpenApi,
};
use sqlx::PgPool;

use crate::{services::shortener::ShortenError, services::shortener::ShortenerService};

#[derive(serde::Deserialize, Object, Debug)]
struct ShortenRequest {
    /// The url to shorten.
    url: String,
}

/// Response for the shorten route.
#[derive(ApiResponse)]
enum ShortenResponse {
    /// The id of the shortened url.
    ///
    /// # Example
    ///
    /// ```text/plain
    /// 3n4j5d
    /// ```
    ///
    /// This is a 6 character alphanumeric string.
    /// To make this into a Url, you need to prepend the domain name like this:
    ///
    /// ```text/plain
    /// https://chibi.shuttleapp.rs/3n4j5d
    /// # OR
    /// https://chibi.shuttleapp.rs/api/3n4j5d
    /// ```
    #[oai(status = 200)]
    Success(PlainText<String>),

    /// An unexpected database error occurred. This is returned when the database returns an error
    ///
    /// More information about the error is included in the response body.
    #[oai(status = 500)]
    DBError(PlainText<String>),

    /// The url was invalid.
    ///
    /// This is returned when the url is not a valid url.
    #[oai(status = 400)]
    InvalidUrl,
}

/// Response for the redirect route.
#[derive(ApiResponse)]
enum RedirectResponse {
    /// The url was found, and the client will be redirected to it.
    #[oai(status = 302)]
    Success(#[oai(header = "Location")] String),

    /// The url was not found.
    #[oai(status = 404)]
    NotFound,

    /// Some database error occurred.
    ///
    /// More information about the error is included in the response body.
    #[oai(status = 500)]
    DBError(PlainText<String>),
}

pub(crate) struct Api;

#[OpenApi]
impl Api {
    /// Shorten a url.
    ///
    /// # Body
    ///
    /// A form with a single field, `url`, which is the url to shorten.
    ///
    /// # Example
    ///
    /// To create a shortened url for `https://google.com/`, you would send a POST request to `/api/` with
    /// the body as `url=https://google.com/`.
    ///
    /// ```bash
    /// curl -X 'POST' \
    ///     'https://chibi.shuttleapp.rs/api/' \
    ///     -H 'accept: text/plain; charset=utf-8' \
    ///     -H 'Content-Type: text/plain; charset=utf-8' \
    ///     -d 'url=https://google.com/'
    /// ```
    ///
    /// Response Header:
    ///
    /// ```yaml
    /// content-length: 6
    /// content-type: text/plain; charset=utf-8
    /// date: Fri,05 May 2023 15:09:40 GMT
    /// ```
    ///
    /// Response Body:
    ///
    /// ```
    /// hREKMY
    /// ```
    ///
    /// URL that redirects to the original URL:
    ///
    /// ```
    /// https://chibi.shuttleapp.rs/hREKMY
    /// ```
    ///
    ///
    #[oai(path = "/", method = "post")]
    async fn shorten(
        &self,
        form: Form<ShortenRequest>,
        shortener_service: Data<&ShortenerService>,
    ) -> ShortenResponse {
        match shortener_service.shorten(&form.url).await {
            Ok(id) => ShortenResponse::Success(PlainText(id)),
            Err(ShortenError::DatabaseError(e)) => {
                ShortenResponse::DBError(PlainText(e.to_string()))
            }
            Err(ShortenError::InvalidUrl) => ShortenResponse::InvalidUrl,
        }
    }

    /// Redirect to a url. A similar route is available at `/{id}`, without the `/api` prefix.
    ///
    /// The route at `/{id}`(without the `/api` prefix) is used for redirecting to the original url, however, that route
    /// is not documented in the OpenAPI spec, and returns less information on errors.
    ///
    /// This route is documented in the OpenAPI spec, and returns more information on errors.
    ///
    /// You should use the route at `/{id}`(without the `/api` prefix) on the client side, as it is shorter.
    ///
    /// # Example
    ///
    /// To redirect to the url with id `hREKMY`, you would send a GET request to `/api/hREKMY` or `/hREKMY/`.
    ///
    /// ```bash
    /// curl -X 'GET' \
    ///     'https://chibi.shuttleapp.rs/api/hREKMY' \
    ///     -H 'accept: */*'
    /// ```
    ///
    /// You should get a 302 redirect to `https://google.com/`.
    ///
    #[oai(path = "/:id", method = "get")]
    async fn redirect(
        &self,
        /// The id of the url to redirect to.
        id: Path<String>,
        shortener_service: Data<&ShortenerService>,
    ) -> RedirectResponse {
        match shortener_service.redirect(&id.0).await {
            Ok(Some(url)) => RedirectResponse::Success(url),
            Ok(None) => RedirectResponse::NotFound,
            Err(e) => {
                RedirectResponse::DBError(PlainText(format!("Database error: {}", e.to_string())))
            }
        }
    }
}
