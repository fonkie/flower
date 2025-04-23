# flower - Rust Implementation

A RESTful API for querying WordPress content, built with Rust, Actix Web 4.0, and Sea-ORM 1.1. This is a read-only API that provides endpoints to retrieve posts, pages, products, customer testimonials, news articles, and categories from a WordPress database.

## Features

- Retrieve posts with filtering and pagination
- Get metadata for specific posts
- List available post types with counts
- Retrieve posts of specific types
- List categories and their associated posts
- Structured, consistent API responses
- OpenAPI-compatible design
- High performance with asynchronous execution
- Dynamic version information pulled directly from Cargo.toml

## Prerequisites

- Rust 1.60 or higher
- MySQL database with WordPress schema
- Docker (optional, for containerized deployment)

## Quick Start

1. Clone the repository:

```bash
git clone https://github.com/yourusername/flower.git
cd flower
```

2. Configure the database connection in the `.env` file:

```
DATABASE_URL=mysql://username:password@host:port/database
SERVER_HOST=127.0.0.1
SERVER_PORT=10086
RUST_LOG=info
CORS_ALLOWED_ORIGIN=*
```

3. Build and run the application:

```bash
cargo build --release
./target/release/flower
```

Alternatively, use Docker Compose:

```bash
docker-compose up -d
```

4. The API is now available at `http://localhost:10086`.

5. Check the API version (which is automatically pulled from Cargo.toml):

```bash
curl -X GET "http://localhost:10086/" -H "accept: application/json"
```

## API Usage Examples

### Basic Information
Get API information:
```bash
curl -X GET "http://localhost:10086/" -H "accept: application/json"
```

### Posts
Get all published posts (with default pagination):
```bash
curl -X GET "http://localhost:10086/api/v1/posts" -H "accept: application/json"
```

Get posts with pagination:
```bash
curl -X GET "http://localhost:10086/api/v1/posts?page=2&page_size=5" -H "accept: application/json"
```

Get a specific post:
```bash
curl -X GET "http://localhost:10086/api/v1/posts/1" -H "accept: application/json"
```

Search posts containing "干细胞" (stem cell):
```bash
curl -X GET "http://localhost:10086/api/v1/posts?search=干细胞" -H "accept: application/json"
```

### Post Types
Get all post types:
```bash
curl -X GET "http://localhost:10086/api/v1/post-types" -H "accept: application/json"
```

Get all pages:
```bash
curl -X GET "http://localhost:10086/api/v1/post-types/page/posts" -H "accept: application/json"
```

Get all products:
```bash
curl -X GET "http://localhost:10086/api/v1/post-types/products/posts" -H "accept: application/json"
```

Get all news articles:
```bash
curl -X GET "http://localhost:10086/api/v1/post-types/news/posts" -H "accept: application/json"
```

Get all customer testimonials:
```bash
curl -X GET "http://localhost:10086/api/v1/post-types/customer/posts" -H "accept: application/json"
```

### Post Metadata
Get metadata for a post:
```bash
curl -X GET "http://localhost:10086/api/v1/posts/1/meta" -H "accept: application/json"
```

### Categories
Get all categories:
```bash
curl -X GET "http://localhost:10086/api/v1/categories" -H "accept: application/json"
```

Get posts in a specific category:
```bash
curl -X GET "http://localhost:10086/api/v1/categories/2/posts" -H "accept: application/json"
```

### Advanced Queries
Get draft posts:
```bash
curl -X GET "http://localhost:10086/api/v1/posts?post_status=draft" -H "accept: application/json"
```

Get posts by a specific author:
```bash
curl -X GET "http://localhost:10086/api/v1/posts?author_id=1" -H "accept: application/json"
```

Get posts with multiple filters:
```bash
curl -X GET "http://localhost:10086/api/v1/posts?post_type=post&post_status=publish&author_id=1&search=wordpress" -H "accept: application/json"
```

### Usage with the Sanyuan Website Data
To view the "走进三源长生" (About Sanyuan) page:
```bash
curl -X GET "http://localhost:10086/api/v1/posts/11" -H "accept: application/json"
```

To get all products from the Sanyuan website:
```bash
curl -X GET "http://localhost:10086/api/v1/post-types/products/posts?page_size=100" -H "accept: application/json"
```

To search for news articles containing "干细胞" (stem cell):
```bash
curl -X GET "http://localhost:10086/api/v1/post-types/news/posts?search=干细胞" -H "accept: application/json"
```

To get customer testimonials for a better display on the website:
```bash
curl -X GET "http://localhost:10086/api/v1/post-types/customer/posts" -H "accept: application/json"
```

## Version Management

The project uses Rust's `env!` macro to dynamically access version information from Cargo.toml at compile time. This ensures:

- The reported API version always matches the package version
- Version information is centralized in a single location (Cargo.toml)
- No need to manually update version strings throughout the codebase

When you update the version in Cargo.toml, simply rebuild the project and the new version will be automatically reflected in the API responses.

## API Endpoints

### Root

```
GET /
```

Returns basic information about the API, including the current version pulled from Cargo.toml.

### Posts

```
GET /api/v1/posts
```

Parameters:
- `post_type`: Filter by post type (post, page, etc.)
- `post_status`: Filter by post status (default: publish)
- `page`: Page number (default: 1)
- `page_size`: Items per page (default: 10, max: 100)
- `search`: Search in post title and content
- `author_id`: Filter by author ID

```
GET /api/v1/posts/{post_id}
```

Get a specific post by ID.

```
GET /api/v1/posts/{post_id}/meta
```

Get metadata for a specific post.

### Post Types

```
GET /api/v1/post-types
```

Get a list of all available post types.

```
GET /api/v1/post-types/{post_type}/posts
```

Parameters:
- `page`: Page number (default: 1)
- `page_size`: Items per page (default: 10, max: 100)
- `post_status`: Filter by post status (default: publish)

### Categories

```
GET /api/v1/categories
```

Parameters:
- `page`: Page number (default: 1)
- `page_size`: Items per page (default: 20, max: 100)

```
GET /api/v1/categories/{category_id}/posts
```

Parameters:
- `page`: Page number (default: 1)
- `page_size`: Items per page (default: 10, max: 100)

## Response Format

All list endpoints return a consistent paginated response format:

```json
{
  "items": [...],  // Array of results
  "total": 42,     // Total items matching the query
  "page": 1,       // Current page number
  "size": 10,      // Items per page
  "pages": 5,      // Total number of pages
  "has_next": true, // Whether there are more pages
  "has_prev": false // Whether there are previous pages
}
```

## Error Handling

The API uses standard HTTP status codes and returns consistent error responses:

```json
{
  "detail": "Error message describing what went wrong"
}
```

Common status codes:
- 200: Success
- 400: Bad Request (invalid parameters)
- 404: Not Found (resource not found)
- 500: Server Error

## Project Structure

```
flower/
├── Cargo.toml                 # Project dependencies and version information
├── .env                       # Environment variables
├── src/
│   ├── main.rs                # Application entry point
│   ├── config.rs              # Configuration handling
│   ├── error.rs               # Error handling
│   ├── api/                   # API endpoints
│   │   ├── mod.rs             
│   │   ├── handlers.rs        # Request handlers (with dynamic version info)
│   │   ├── routes.rs          # Route definitions
│   │   └── responses.rs       # Response models
│   ├── db/                    # Database interactions
│   │   ├── mod.rs             
│   │   ├── connection.rs      # Database connection
│   │   └── queries.rs         # Database query functions
│   └── models/                # Entity models
│       ├── mod.rs             
│       ├── post.rs            # Post model
│       ├── postmeta.rs        # Post metadata model
│       ├── term.rs            # Terms (categories) model
│       ├── term_relationship.rs # Term relationships model
│       └── term_taxonomy.rs   # Term taxonomies model
```

## Performance Considerations

- Connection pooling is used to efficiently manage database connections
- Queries are designed to be efficient and leverage indexes
- Pagination is implemented to limit result sizes and improve performance
- Asynchronous operations keep the server responsive under load

## License

This project is licensed under the GNU General Public License v3.0 - see the full license text below.

Copyright (C) 2025 flower Contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.