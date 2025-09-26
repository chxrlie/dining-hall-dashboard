# Dining Hall Dashboard

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/chxrlie/dining-hall-dashboard/actions)
[![Dependencies](https://img.shields.io/badge/dependencies-up%20to%20date-brightgreen.svg)](Cargo.toml)

A modern, Rust-based web application for managing dining hall menu items, notices, and schedules with a comprehensive admin interface.

## 🎯 Value Proposition

The Dining Hall Dashboard streamlines dining facility operations by providing an intuitive interface for menu management, automated scheduling, and real-time notice updates. Built with performance and security in mind, it offers a seamless experience for both administrators and diners.

## 📖 Table of Contents

- [Quick Start](#-quick-start)
- [API Documentation](API.md)
- [Design](DESIGN.md)
- [Development Guide](DEVELOPMENT.md)
- [Security](SECURITY.md)
- [Contributing](#-contributing)
- [License](#-license)

## 🚀 Quick Start

### Prerequisites

- Rust 2024 edition or higher and Cargo
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
