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
- Support for international characters in search queries
- Public access through Nginx proxy integration

## Prerequisites

- Rust 1.60 or higher
- MySQL database with WordPress schema
- Docker (optional, for containerized deployment)
- Nginx (optional, for public access configuration)

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

## Deployment Options

### Local Development Access

For local development, the API is accessible at `http://localhost:10086` once the service is running.

### Public Access via Nginx

For production environments, the Flower API can be accessed through a domain name with Nginx as a reverse proxy. This provides several benefits:

- Secure access through SSL/TLS certificates
- Integration with existing websites
- Standard API URL patterns
- Improved security by hiding internal services

With our configured setup, the API is accessible at:
```
https://sanyuan.xn--bww30p.com/api/
```

The Nginx configuration proxies all requests from `/api/` to the internal Flower service running on port 10086. This allows you to access all API endpoints through the main website domain.

## API Usage Examples

The examples below show both local development and public access URLs.

### Basic Information
Get API information:
```bash
# Local development
curl -X GET "http://localhost:10086/" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api" -H "accept: application/json"
```

### Posts
Get all published posts (with default pagination):
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/posts" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/posts" -H "accept: application/json"
```

Get posts with pagination:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/posts?page=2&page_size=5" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/posts?page=2&page_size=5" -H "accept: application/json"
```

Get a specific post:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/posts/1" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/posts/1" -H "accept: application/json"
```

Search posts containing "干细胞" (stem cell):
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/posts?search=%E5%B9%B2%E7%BB%86%E8%83%9E" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/posts?search=%E5%B9%B2%E7%BB%86%E8%83%9E" -H "accept: application/json"
```

### Post Types
Get all post types:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/post-types" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/post-types" -H "accept: application/json"
```

Get all pages:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/post-types/page/posts" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/post-types/page/posts" -H "accept: application/json"
```

Get all products:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/post-types/products/posts" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/post-types/products/posts" -H "accept: application/json"
```

Get all news articles:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/post-types/news/posts" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/post-types/news/posts" -H "accept: application/json"
```

Get all customer testimonials:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/post-types/customer/posts" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/post-types/customer/posts" -H "accept: application/json"
```

### Post Metadata
Get metadata for a post:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/posts/1/meta" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/posts/1/meta" -H "accept: application/json"
```

### Categories
Get all categories:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/categories" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/categories" -H "accept: application/json"
```

Get posts in a specific category:
```bash
# Local development
curl -X GET "http://localhost:10086/api/v1/categories/2/posts" -H "accept: application/json"

# Public access
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/categories/2/posts" -H "accept: application/json"
```

## Nginx Configuration for Public Access

Below is the Nginx configuration that enables public access to the Flower API:

```nginx
# Flower API proxy - Location block for the API
location /api/ {
    proxy_pass http://127.0.0.1:10086/api/;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
    proxy_buffering off;
    proxy_read_timeout 90s;
    client_max_body_size 10m;
    
    # Add CORS headers for API requests
    add_header 'Access-Control-Allow-Origin' '*' always;
    add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS' always;
    add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization' always;
    
    # Handle preflight requests
    if ($request_method = 'OPTIONS') {
        add_header 'Access-Control-Allow-Origin' '*';
        add_header 'Access-Control-Allow-Methods' 'GET, POST, OPTIONS';
        add_header 'Access-Control-Allow-Headers' 'DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range,Authorization';
        add_header 'Access-Control-Max-Age' 1728000;
        add_header 'Content-Type' 'text/plain; charset=utf-8';
        add_header 'Content-Length' 0;
        return 204;
    }
}

# Root API info endpoint
location = /api {
    proxy_pass http://127.0.0.1:10086/;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
}
```

This configuration should be added to your website's server block in the Nginx configuration. It handles:

1. Proxying all `/api/` requests to the Flower API
2. Setting up proper headers for CORS support
3. Handling OPTIONS preflight requests for cross-origin requests
4. Converting the root API endpoint (`/api`) to the Flower API's root endpoint (`/`)

## International Character Support

The Flower API fully supports non-ASCII characters in search queries, including Chinese characters. When using characters outside the standard ASCII set in URLs (like in search parameters), they must be properly URL-encoded to comply with HTTP standards.

### URL Encoding for Non-ASCII Characters

When searching for terms with non-ASCII characters, you must URL encode those characters. For example, the Chinese term "干细胞" (stem cell) should be encoded as "%E5%B9%B2%E7%BB%86%E8%83%9E" in a URL.

You can use built-in URL encoding functions in your programming language:

**Bash:**
```bash
# URL encode function for bash
urlencode() {
  local string="$1"
  local length="${#string}"
  local encoded=""
  local pos c o
  
  for (( pos=0; pos<length; pos++ )); do
    c="${string:$pos:1}"
    case "$c" in
      [-_.~a-zA-Z0-9]) # RFC 3986 unreserved characters
        encoded+="$c"
        ;;
      *)
        printf -v o '%%%02X' "'$c"
        encoded+="$o"
        ;;
    esac
  done
  echo "${encoded}"
}

# Example usage
SEARCH_TERM="干细胞"
ENCODED_TERM=$(urlencode "$SEARCH_TERM")
curl -X GET "https://sanyuan.xn--bww30p.com/api/v1/posts?search=${ENCODED_TERM}" -H "accept: application/json"
```

**Python:**
```python
import urllib.parse

search_term = "干细胞"
encoded_term = urllib.parse.quote(search_term)
# encoded_term will be '%E5%B9%B2%E7%BB%86%E8%83%9E'
```

**JavaScript:**
```javascript
const searchTerm = "干细胞";
const encodedTerm = encodeURIComponent(searchTerm);
// encodedTerm will be '%E5%B9%B2%E7%BB%86%E8%83%9E'
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
- `search`: Search in post title and content

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
- `search`: Search in post title and content

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
- Nginx reverse proxy provides additional caching and performance benefits

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