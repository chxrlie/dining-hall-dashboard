# 🛠️ Development Guide

## Initial Setup

Before running the application for the first time, you need to ensure that the `data` directory has the correct permissions. The application needs to be able to write to this directory to store its data.

On Windows, you can set the correct permissions by running the following command in the project's root directory:

```bash
icacls "data" /grant "%USERNAME%":(F) /T
```

This command grants your user account full control over the `data` directory and all of its contents.

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
