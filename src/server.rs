use crate::{
    builder::build_site,
    cli::{self, Cli},
    log,
};
use anyhow::{Context, Result};
use axum::{
    Router,
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::{get, get_service},
};
use core::panic;
use std::{
    fs,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub async fn start_server(cli: &'static Cli) -> Result<()> {
    build_site(cli)?;
    // println!("AAAAAAAAAAAAAA");

    let Some(cli::Commands::Serve { interface, port, .. }) = &cli.command else {
        panic!("Wrong internal implementation, I think this wouldn't occur")
    };
    let interface = IpAddr::from_str(interface)?;
    let port = *port;

    let addr = SocketAddr::new(interface, port);

    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("[Server] Failed to bind to address {}", addr))?;

    let app = {
        let base_path = cli.output_dir.clone();
        let serve_dir = ServeDir::new(&cli.output_dir)
            .append_index_html_on_directories(false)
            .not_found_service(get(move |url| handle_path(url, base_path)));
        Router::new().fallback(get_service(serve_dir))
    };

    log!("server", "Serving site on http://{}", addr);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("[Server] Failed to start")?;

    Ok(())
}

async fn handle_path(uri: Uri, base_path: PathBuf) -> impl IntoResponse {
    let request_path = uri.path().trim_start_matches('/');
    let local_path = base_path.join(request_path);

    if local_path.is_file() {
        return match fs::read_to_string(&local_path) {
            Ok(content) => Html(content).into_response(),
            Err(_) => handle_404().await.into_response(),
        };
    }
    if local_path.is_dir() {
        let index_path = local_path.join("index.html");
        if index_path.is_file() {
            return match fs::read_to_string(&index_path) {
                Ok(content) => Html(content).into_response(),
                Err(_) => handle_404().await.into_response(),
            };
        }
        let mut file_list = String::new();
        if let Ok(entries) = fs::read_dir(&local_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().into_owned();
                let href = format!("{}/{}", uri.path().trim_end_matches('/'), name);
                file_list.push_str(&format!("<li><a href='{}'>{}</a></li>", href, name));
            }
            return Html(format!(
                "<html><body><h1>Directory: {}</h1><ul>{}</ul></body></html>",
                uri.path(),
                file_list
            ))
            .into_response();
        }
    }
    handle_404().await.into_response()
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 Not Found")
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
    log!("server", "Shutting down gracefully...");
}
