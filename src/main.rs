use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use bytes::Bytes;
use serde::Serialize;
use tower_http::cors::CorsLayer;
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, ToSchema)]
struct IntensityResponse {
    /// The calculated average intensity value (0-255)
    average_intensity: f64,
    /// Success message with formatted intensity value
    message: String,
}

#[derive(Serialize, ToSchema)]
struct ErrorResponse {
    /// Error description
    error: String,
}

#[derive(OpenApi)]
#[openapi(
    paths(calculate_intensity, health_check),
    components(schemas(IntensityResponse, ErrorResponse)),
    tags(
        (name = "Image Processing", description = "Image intensity calculation API")
    ),
    info(
        title = "Web Image Intensity Calculator API",
        description = "A REST API for calculating the average intensity of uploaded images",
        version = "1.0.0"
    )
)]
struct ApiDoc;

#[utoipa::path(
    post,
    path = "/calculate-intensity",
    tag = "Image Processing",
    request_body(
        content = String,
        description = "Image file uploaded as multipart/form-data with field name 'image'",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successfully calculated image intensity", body = IntensityResponse),
        (status = 400, description = "Bad request - invalid or missing image data"),
        (status = 422, description = "Unprocessable entity - invalid image format")
    )
)]
async fn calculate_intensity(mut multipart: Multipart) -> Result<Json<IntensityResponse>, StatusCode> {
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        if field.name() == Some("image") {
            let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
            
            match calculate_image_intensity(data) {
                Ok(intensity) => {
                    return Ok(Json(IntensityResponse {
                        average_intensity: intensity,
                        message: format!("Average intensity calculated: {:.2}", intensity),
                    }));
                }
                Err(_) => return Err(StatusCode::UNPROCESSABLE_ENTITY),
            }
        }
    }
    
    Err(StatusCode::BAD_REQUEST)
}

fn calculate_image_intensity(image_data: Bytes) -> Result<f64, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(&image_data)?;
    let rgb_img = img.to_rgb8();
    
    let mut total_intensity = 0u64;
    let mut pixel_count = 0u64;
    
    for pixel in rgb_img.pixels() {
        let r = pixel[0] as u64;
        let g = pixel[1] as u64;
        let b = pixel[2] as u64;
        
        let intensity = (r + g + b) / 3;
        total_intensity += intensity;
        pixel_count += 1;
    }
    
    if pixel_count == 0 {
        return Err("No pixels found in image".into());
    }
    
    Ok(total_intensity as f64 / pixel_count as f64)
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = String)
    )
)]
async fn health_check() -> &'static str {
    "OK"
}

async fn serve_swagger() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>API Documentation</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css" />
    <style>
        html { box-sizing: border-box; overflow: -moz-scrollbars-vertical; overflow-y: scroll; }
        *, *:before, *:after { box-sizing: inherit; }
        body { margin:0; background: #fafafa; }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-standalone-preset.js"></script>
    <script>
        window.onload = function() {
            const ui = SwaggerUIBundle({
                url: '/api-docs/openapi.json',
                dom_id: '#swagger-ui',
                deepLinking: true,
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIStandalonePreset
                ],
                plugins: [
                    SwaggerUIBundle.plugins.DownloadUrl
                ],
                layout: "StandaloneLayout"
            });
        };
    </script>
</body>
</html>
    "#)
}

async fn serve_openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/calculate-intensity", post(calculate_intensity))
        .route("/health", get(health_check))
        .route("/swagger-ui", get(serve_swagger))
        .route("/api-docs/openapi.json", get(serve_openapi))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    println!("POST /calculate-intensity - Upload an image to calculate average intensity");
    println!("GET  /health - Health check endpoint");
    println!("GET  /swagger-ui - Swagger documentation UI");
    
    axum::serve(listener, app).await.unwrap();
}
