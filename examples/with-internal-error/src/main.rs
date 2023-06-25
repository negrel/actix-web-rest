use actix_web::{web, App, HttpResponse, HttpServer, ResponseError};
use actix_web_rest::{http::StatusCode, rest_error};
use anyhow::anyhow;

#[allow(clippy::enum_variant_names)]
#[rest_error(internal_error)]
enum MyEndpointError {
    #[rest(status_code = StatusCode::BAD_REQUEST)]
    #[error("error foo")]
    FooError,

    #[rest(status_code = StatusCode::CONFLICT)]
    #[error("error bar")]
    BarError,
}

async fn handler(path: web::Path<String>) -> Result<HttpResponse, impl ResponseError> {
    let path_param = path.into_inner();
    match path_param.as_ref() {
        "foo" => Err(MyEndpointError::FooError),
        "bar" => Err(MyEndpointError::BarError),
        _ => Err(MyEndpointError::InternalError(anyhow!(
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
