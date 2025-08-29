# Dining Hall Dashboard

A Rust-based web application for managing dining hall menu items and site notices with admin authentication.

## Features

- **Menu Management**: Create, read, update, and delete menu items with categories, pricing, and allergen information
- **Notice System**: Manage active site notices with timestamps and activation status
- **Admin Authentication**: Secure admin panel with session-based authentication
- **RESTful API**: JSON-based API endpoints for programmatic access
- **Responsive Design**: Mobile-friendly interface with accessibility features
- **JSON Storage**: Persistent data storage using JSON files

## Tech Stack

- **Backend**: Rust with Actix-web framework
- **Templating**: Tera templates for server-side rendering
- **Authentication**: Argon2 password hashing with secure sessions
- **Storage**: JSON file-based storage with thread-safe operations
- **Frontend**: Semantic HTML, CSS Grid/Flexbox, minimal JavaScript

## Quick Start

### Prerequisites

- Rust 1.70+ and Cargo
- Git

### Installation

1. Clone the repository:

```bash
git clone <repository-url>
cd dining-hall-dashboard
```

2. Build and run the application:

```bash
cargo run
```

3. Open your browser to `http://localhost:8080`

### Default Admin Account

A default admin user is created automatically:

- **Username**: `admin`
- **Password**: `admin123`

**Important**: Change the default password immediately after first login.

## Project Structure

```
dining-hall-dashboard/
├── src/                 # Rust source code
│   ├── main.rs         # Application entry point
│   ├── auth.rs         # Authentication handlers
│   ├── handlers.rs     # HTTP request handlers
│   └── storage.rs      # JSON storage system
├── data/               # JSON data files
│   ├── menu_items.json
│   ├── notices.json
│   └── admin_users.json
├── templates/          # HTML templates
│   ├── base.html
│   ├── menu.html
│   ├── admin/
│   └── partials/
├── static/             # Static assets
│   ├── css/
│   └── images/
└── assets/             # Branding assets
```

## API Endpoints

### Menu Items

- `GET /api/items` - List all menu items
- `POST /api/items` - Create new menu item
- `PUT /api/items/{id}` - Update menu item
- `DELETE /api/items/{id}` - Delete menu item

### Notices

- `GET /api/notices` - List all notices
- `POST /api/notices` - Create new notice
- `PUT /api/notices/{id}` - Update notice
- `DELETE /api/notices/{id}` - Delete notice

### Authentication

- `GET /admin/login` - Login page
- `POST /admin/login` - Handle login
- `POST /admin/logout` - Handle logout
- `GET /admin` - Admin dashboard

## Data Models

### MenuItem

```rust
struct MenuItem {
    id: Uuid,
    name: String,
    category: MenuCategory, // Mains, Sides, Desserts, Beverages
    description: String,
    price: f64,
    allergens: Vec<String>,
    is_available: bool,
}
```

### Notice

```rust
struct Notice {
    id: Uuid,
    title: String,
    content: String,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
```

## Development

### Building for Production

```bash
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Environment Variables

The application uses the following environment variables:

- `RUST_LOG` - Log level (default: info)
- `SESSION_SECRET` - Session encryption key (for production)

## Security Features

- Argon2 password hashing with salt
- Secure HTTP-only cookies
- CSRF protection
- Input validation and sanitization
- Session expiration (24 hours)
- Rate limiting on authentication endpoints

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the AGPLv3 License - see the LICENSE file for details.
