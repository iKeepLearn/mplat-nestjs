use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Result,
    dev::ServerHandle,
    error::InternalError,
    http::StatusCode,
    middleware::{self, Compress, from_fn},
    web::{self, FormConfig, JsonConfig, QueryConfig},
};
use config::AppConfig;
use error::{Error, ErrorResponse};
use routes::middleware::{async_auth_middleware, rate_limit::RateLimit};
use routes::{AppState, config};
use sqlx::{Pool, Postgres};
use tokio::signal::unix::SignalKind;

pub async fn start_server(
    config: AppConfig,
    db_pool: Pool<Postgres>,
    port: u16,
    front_end_host: String,
) -> Result<(), Error> {
    let app_state = AppState::new(db_pool, config);
    let rate_limit = RateLimit::with_default_config();
    let server = create_server(app_state, rate_limit, port, front_end_host)?;

    let mut interrupt = tokio::signal::unix::signal(SignalKind::interrupt())?;
    let mut terminate = tokio::signal::unix::signal(SignalKind::terminate())?;

    tokio::select! {
      _ = tokio::signal::ctrl_c() => {
        tracing::warn!("Received ctrl-c, shutting down gracefully...");
      }
      _ = interrupt.recv() => {
        tracing::warn!("Received interrupt, shutting down gracefully...");
      }
      _ = terminate.recv() => {
        tracing::warn!("Received terminate, shutting down gracefully...");
      }
    }

    server.stop(true).await;

    Ok(())
}

fn json_config() -> JsonConfig {
    JsonConfig::default().error_handler(|err, _req| {
        let error_response = ErrorResponse::new(StatusCode::BAD_REQUEST.as_u16(), &err.to_string());
        InternalError::from_response(err, HttpResponse::Ok().json(error_response)).into()
    })
}

fn form_config() -> FormConfig {
    FormConfig::default().error_handler(|err, _req| {
        let error_response = ErrorResponse::new(StatusCode::BAD_REQUEST.as_u16(), &err.to_string());
        InternalError::from_response(err, HttpResponse::Ok().json(error_response)).into()
    })
}

fn query_config() -> QueryConfig {
    QueryConfig::default().error_handler(|err, _req| {
        let error_response = ErrorResponse::new(StatusCode::BAD_REQUEST.as_u16(), &err.to_string());
        InternalError::from_response(err, HttpResponse::Ok().json(error_response)).into()
    })
}

fn create_server(
    app_state: AppState,
    rate_limit: RateLimit,
    port: u16,
    front_end_host: String,
) -> Result<ServerHandle, Error> {
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&front_end_host) // 允许任何域名访问
            .allow_any_method() // 允许任何 HTTP 方法 (GET, POST, OPTIONS 等)
            .allow_any_header() // 允许任何请求头
            .supports_credentials(); // 允许携带 Cookie (如果需要)
        // .max_age(3600); // 预检请求结果缓存1小时，减少请求次数
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::new(
                // This is the default log format save for the usage of %{r}a over %a to guarantee to
                // record the client's (forwarded) IP and not the last peer address, since the latter is
                // frequently just a reverse proxy
                "%{r}a '%r' %s %b '%{Referer}i' '%{User-Agent}i' '%{request-id}i' %T",
            ))
            .wrap(Compress::default())
            .wrap(from_fn(async_auth_middleware))
            //.wrap(middleware::from_fn(convert_response))
            .app_data(web::Data::new(app_state.clone()))
            .app_data(json_config())
            .app_data(form_config())
            .app_data(query_config())
            .configure(|cfg| config(cfg, &rate_limit))
    })
    .bind(("0.0.0.0", port))?
    .run();

    let handler = server.handle();
    tokio::task::spawn(server);
    Ok(handler)
}
