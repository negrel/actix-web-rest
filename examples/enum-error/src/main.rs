use actix_web::{web, App, HttpResponse, HttpServer, ResponseError};
use actix_web_rest::{actix_web, http::StatusCode, rest_error};
use anyhow::anyhow;

#[allow(clippy::enum_variant_names)]
#[rest_error]
enum MyEndpointError {
    #[rest(status_code = StatusCode::OK)]
    #[error("error foo")]
    FooError,

    #[rest(status_code = StatusCode::OK)]
    #[error("error bar")]
    BarError,

    #[rest(status_code = StatusCode::INTERNAL_SERVER_ERROR)]
    #[error(transparent)]
    #[serde(skip)]
    UnexpectedError(#[from] anyhow::Error),
}

async fn handler(path: web::Path<String>) -> Result<HttpResponse, impl ResponseError> {
    let path_param = path.into_inner();
    match path_param.as_ref() {
        "foo" => Err(MyEndpointError::FooError),
        "bar" => Err(MyEndpointError::BarError),
        _ => Err(MyEndpointError::from(anyhow!(
            "unexpected path params: {path_param}"
        ))),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // curl http://localhost:8080/foo
    // curl http://localhost:8080/bar
    // curl http://localhost:8080/baz
    HttpServer::new(|| App::new().route("/{error}", web::get().to(handler)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
