use axum::{http::StatusCode, response::{IntoResponse, Response}, routing::post, Router};
use axum_extra::extract::multipart::Multipart;
use image::{ImageOutputFormat, DynamicImage};
use std::net::SocketAddr;
use std::io::Cursor;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/process", post(process_image));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn process_image(mut multipart: Multipart) -> Response {
    // Only handle the first file part
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(_file_name) = field.file_name() {
            let _content_type = field.content_type().unwrap_or("");
            let data = field.bytes().await.unwrap();
            // Process image
            match image::load_from_memory(&data) {
                Ok(img) => {
                    let gray = img.grayscale();
                    let mut buf = Cursor::new(Vec::new());
                    if gray.write_to(&mut buf, ImageOutputFormat::Png).is_ok() {
                        let bytes = buf.into_inner();
                        return (
                            StatusCode::OK,
                            [("Content-Type", "image/png")],
                            bytes
                        ).into_response();
                    } else {
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Failed to encode image"
                        ).into_response();
                    }
                }
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        "Invalid image file"
                    ).into_response();
                }
            }
        }
    }
    (StatusCode::BAD_REQUEST, "No image file found").into_response()
}
