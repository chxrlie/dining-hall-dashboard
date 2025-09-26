# 🛠️ Development Guide

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
│   ├── error_handler.rs # Application error handlers
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

### Data Management

The application uses a JSON-based file system for data storage, located in the `data/` directory. Each file corresponds to a specific data model:

- `admin_users.json`: Stores administrator credentials.
- `menu_items.json`: Contains all menu items.
- `notices.json`: Holds announcements and notices.
- `menu_presets.json`: Defines reusable collections of menu items.
- `menu_schedules.json`: Manages the scheduling of menu presets.

These files can be manually edited, but it is recommended to use the admin interface to ensure data integrity.
