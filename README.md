# Dining Hall Dashboard

[![Build Status](https://img.shields.io/badge/build-not%20configured-orange.svg)](https://github.com/your-org/dining-hall-dashboard/actions)
[![Dependencies](https://img.shields.io/badge/dependencies-up%20to%20date-brightgreen.svg)](Cargo.toml)

A modern, Rust-based web application for managing dining hall menu items, notices, and schedules with a comprehensive admin interface.

## 🎯 Value Proposition

The Dining Hall Dashboard streamlines dining facility operations by providing an intuitive interface for menu management, automated scheduling, and real-time notice updates. Built with performance and security in mind, it offers a seamless experience for both administrators and diners.

## 📖 Table of Contents

- [Features](#-features)
- [Quick Start](#-quick-start)
- [API Documentation](#-api-documentation)
- [Development Guide](#-development-guide)
- [Security Features](#-security-features)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

### Menu Management

- **Comprehensive CRUD Operations**: Create, read, update, and delete menu items with detailed categorization
- **Allergen Tracking**: Comprehensive allergen information for dietary safety
- **Availability Control**: Real-time menu item availability toggling

### Notice System

- **Dynamic Announcements**: Create and manage site notices with activation status
- **Timestamp Tracking**: Automatic creation and update timestamps for notices

### Admin Authentication

- **Secure Session Management**: Cookie-based authentication with secure session handling
- **Argon2 Password Hashing**: Industry-standard password security
- **CSRF Protection**: Built-in cross-site request forgery protection

### Scheduling System

- **Menu Presets**: Create reusable menu configurations
- **Automated Scheduling**: Schedule menu changes with recurring options
- **Conflict Detection**: Automatic schedule conflict detection and resolution

### RESTful API

- **JSON-based Endpoints**: Consistent API responses for programmatic access
- **Error Handling**: Comprehensive error responses with appropriate HTTP status codes

### Responsive Design

- **Mobile-First Approach**: Fully responsive interface for all devices
- **Accessibility Features**: WCAG 2.1 AA compliant design
- **Modern UI Components**: Clean, intuitive user interface

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ and Cargo
- Git

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/chxrlie/dining-hall-dashboard.git
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

⚠️ **Important**: Change the default password immediately after first login for security.

## 📡 API Documentation

The application provides a comprehensive RESTful API for programmatic access to all features.

### Authentication Endpoints

| Method | Endpoint        | Description        |
| ------ | --------------- | ------------------ |
| `GET`  | `/admin/login`  | Display login page |
| `POST` | `/admin/login`  | Authenticate user  |
| `POST` | `/admin/logout` | End user session   |

### Menu Item Endpoints

| Method   | Endpoint          | Description          |
| -------- | ----------------- | -------------------- |
| `GET`    | `/api/items`      | List all menu items  |
| `POST`   | `/api/items`      | Create new menu item |
| `PUT`    | `/api/items/{id}` | Update menu item     |
| `DELETE` | `/api/items/{id}` | Delete menu item     |

### Notice Endpoints

| Method   | Endpoint            | Description       |
| -------- | ------------------- | ----------------- |
| `GET`    | `/api/notices`      | List all notices  |
| `POST`   | `/api/notices`      | Create new notice |
| `PUT`    | `/api/notices/{id}` | Update notice     |
| `DELETE` | `/api/notices/{id}` | Delete notice     |

### Menu Preset Endpoints

| Method   | Endpoint            | Description              |
| -------- | ------------------- | ------------------------ |
| `GET`    | `/api/presets`      | List all menu presets    |
| `POST`   | `/api/presets`      | Create new menu preset   |
| `GET`    | `/api/presets/{id}` | Get specific menu preset |
| `PUT`    | `/api/presets/{id}` | Update menu preset       |
| `DELETE` | `/api/presets/{id}` | Delete menu preset       |

### Schedule Endpoints

| Method   | Endpoint                  | Description                  |
| -------- | ------------------------- | ---------------------------- |
| `GET`    | `/api/schedules`          | List all menu schedules      |
| `POST`   | `/api/schedules`          | Create new menu schedule     |
| `GET`    | `/api/schedules/{id}`     | Get specific menu schedule   |
| `PUT`    | `/api/schedules/{id}`     | Update menu schedule         |
| `DELETE` | `/api/schedules/{id}`     | Delete menu schedule         |
| `GET`    | `/api/schedules/upcoming` | List upcoming schedules      |
| `POST`   | `/api/schedules/validate` | Validate schedule parameters |

### API Response Examples

#### Get All Menu Items

```http
GET /api/items
```

Response:

```json
[
  {
    "id": "11111111-1111-1111-1111-111111111111",
    "name": "Grilled Chicken Breast",
    "category": "Mains",
    "description": "Tender grilled chicken breast with herbs and spices",
    "allergens": ["Soy"],
    "is_available": true
  }
]
```

#### Create New Menu Item

```http
POST /api/items
Content-Type: application/json

{
  "name": "Vegetable Stir Fry",
  "category": "Mains",
  "description": "Fresh seasonal vegetables stir-fried with tofu",
  "allergens": ["Soy", "Gluten"],
  "is_available": true
}
```

Response:

```json
{
  "id": "22222222-2222-2222-2222-222222222222",
  "name": "Vegetable Stir Fry",
  "category": "Mains",
  "description": "Fresh seasonal vegetables stir-fried with tofu",
  "allergens": ["Soy", "Gluten"],
  "is_available": true
}
```

## 🛠️ Development Guide

### Building for Production

```bash
cargo build --release
```

The production binary will be located at `target/release/dining-hall-dashboard`.

### Running Tests

```bash
cargo test
```

### Project Structure

```
dining-hall-dashboard/
├── src/                 # Rust source code
│   ├── main.rs         # Application entry point
│   ├── auth.rs         # Authentication handlers
│   ├── handlers.rs     # HTTP request handlers
│   ├── storage.rs      # JSON storage system
│   ├── scheduler.rs    # Automated scheduling system
├── data/               # JSON data files
│   ├── menu_items.json
│   ├── notices.json
│   ├── admin_users.json
│   ├── menu_presets.json
│   ├── menu_schedules.json
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

### Environment Variables

The application uses the following environment variables:

- `RUST_LOG` - Log level (default: info)
- `SESSION_SECRET` - Session encryption key (for production)

## 🔐 Security Features

The Dining Hall Dashboard implements multiple security measures to protect data and user sessions:

- **Argon2 Password Hashing**: Industry-standard password security with salt
- **Secure HTTP-only Cookies**: Session cookies with HttpOnly and Secure flags
- **CSRF Protection**: Cross-site request forgery protection tokens
- **Input Validation**: Server-side validation and sanitization of all inputs
- **Session Expiration**: Automatic session expiration after 24 hours
- **Rate Limiting**: Built-in rate limiting on authentication endpoints

## 🤝 Contributing

We welcome contributions to the Dining Hall Dashboard! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a pull request

Please ensure your code follows the existing style and includes appropriate tests.

## 📄 License

This project is licensed under the AGPLv3 License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">
  <sub>Built with ❤️ using Rust and Actix-web</sub>
</div>
