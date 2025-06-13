# Web Image Intensity Calculator API

A REST API built in Rust that calculates the average intensity of uploaded images. This service provides a simple HTTP endpoint for image analysis with built-in Swagger documentation.

## Features

- 🖼️ **Image Upload**: Accept images via multipart form data
- 📊 **Intensity Calculation**: Calculate average pixel intensity (0-255 scale)
- 📚 **Swagger Documentation**: Interactive API documentation at `/swagger-ui`
- 🚀 **Fast & Efficient**: Built with Rust and Axum for high performance
- 🌐 **CORS Enabled**: Ready for frontend integration
- ✅ **Health Check**: Monitor service status at `/health`

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `POST` | `/calculate-intensity` | Upload image and get intensity |
| `GET` | `/health` | Service health check |
| `GET` | `/swagger-ui` | Interactive API documentation |
| `GET` | `/api-docs/openapi.json` | OpenAPI specification |

## Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))

### Installation & Usage

1. **Clone and navigate to project**:
   ```bash
   git clone <repository-url>
   cd webcalculation
   ```

2. **Run the server**:
   ```bash
   cargo run
   ```

3. **Server will start on**: `http://localhost:3000`

4. **View API documentation**: `http://localhost:3000/swagger-ui`

### Testing the API

**Using curl**:
```bash
curl -X POST http://localhost:3000/calculate-intensity \
  -F "image=@path/to/your/image.jpg"
```

**Response**:
```json
{
  "average_intensity": 128.75,
  "message": "Average intensity calculated: 128.75"
}
```

## Supported Image Formats

- JPEG/JPG
- PNG
- GIF
- WEBP
- BMP
- TIFF
- And most other common formats supported by the `image` crate

## How It Works

1. **Image Upload**: Client uploads image via multipart form data
2. **Processing**: Server loads image and converts to RGB
3. **Calculation**: Computes average intensity across all pixels
4. **Response**: Returns intensity value (0 = black, 255 = white)

The intensity calculation uses the formula:
```
intensity = (R + G + B) / 3
average = sum(all_pixel_intensities) / total_pixels
```

## Dependencies

- **axum**: Modern web framework for Rust
- **tokio**: Async runtime
- **image**: Image processing library
- **utoipa**: OpenAPI documentation generation
- **serde**: JSON serialization
- **tower-http**: HTTP middleware (CORS)

## Development

### Building
```bash
cargo build
```

### Running tests
```bash
cargo test
```

### Development server with auto-reload
```bash
cargo watch -x run
```

## Error Handling

The API provides clear error responses:

- `400 Bad Request`: Invalid or missing image data
- `422 Unprocessable Entity`: Unsupported image format
- `500 Internal Server Error`: Server processing error

## Frontend Integration

This API is designed to work with web frontends. A companion frontend project is available at [imag2intensity](../imag2intensity/) that provides a minimalist interface for uploading images and displaying results.

## CORS Configuration

CORS is enabled for all origins to support frontend integration. In production, configure specific allowed origins in the CORS middleware.

## License

This project was created as an experiment with Claude Code.
