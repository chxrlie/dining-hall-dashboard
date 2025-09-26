# Dining Hall Dashboard Deployment Guide

## 🚀 Production Deployment

This guide provides instructions for deploying the Dining Hall Dashboard in a production environment.

## 📋 System Requirements

### Minimum Requirements

- **CPU**: 1 core
- **RAM**: 512 MB
- **Disk Space**: 100 MB for application, additional space for data
- **Operating System**: Linux (Ubuntu 20.04+, CentOS 8+, Debian 10+) or Windows Server 2019+
- **Rust**: 2024 edition or higher

### Recommended Requirements

- **CPU**: 2 cores
- **RAM**: 1 GB
- **Disk Space**: 1 GB
- **Operating System**: Ubuntu 22.04 LTS or equivalent

## 🛠️ Installation Methods

### Method 1: Binary Deployment (Recommended)

1. **Download the latest release**

   ```bash
   wget https://github.com/chxrlie/dining-hall-dashboard/releases/latest/download/dining-hall-dashboard.tar.gz
   tar -xzf dining-hall-dashboard.tar.gz
   cd dining-hall-dashboard
   ```

2. **Create configuration directory**

   ```bash
   mkdir -p /etc/dining-hall-dashboard
   cp -r data /etc/dining-hall-dashboard/
   cp -r templates /etc/dining-hall-dashboard/
   cp -r static /etc/dining-hall-dashboard/
   cp -r assets /etc/dining-hall-dashboard/
   ```

3. **Set permissions**
   ```bash
   chown -R www-data:www-data /etc/dining-hall-dashboard
   chmod -R 755 /etc/dining-hall-dashboard
   ```

### Method 2: Building from Source

1. **Install Rust**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Clone and build**

   ```bash
   git clone https://github.com/your-org/dining-hall-dashboard.git
   cd dining-hall-dashboard
   cargo build --release
   ```

3. **Copy files to deployment directory**
   ```bash
   mkdir -p /opt/dining-hall-dashboard
   cp target/release/dining-hall-dashboard /opt/dining-hall-dashboard/
   cp -r data templates static assets /opt/dining-hall-dashboard/
   ```

## ⚙️ Configuration

### Environment Variables

Set these environment variables for production deployment:

```bash
# Logging level (error, warn, info, debug, trace)
export RUST_LOG=info

# Session secret key (generate a secure random key)
export SESSION_SECRET="your-64-character-secret-key-here"

# Port to listen on
export PORT=8080

# Bind address
export BIND_ADDRESS="0.0.0.0"
```

### Generating a Session Secret

Generate a secure session secret:

```bash
# Generate a 64-character hexadecimal key
openssl rand -hex 32
```

### Data Directory Configuration

The application expects the following directory structure:

```
/etc/dining-hall-dashboard/  # or /opt/dining-hall-dashboard/
├── data/
│   ├── menu_items.json
│   ├── notices.json
│   ├── admin_users.json
│   ├── menu_presets.json
│   └── menu_schedules.json
├── templates/
├── static/
└── assets/
```

Ensure the data directory is writable by the application process.

## 🔧 Reverse Proxy Configuration

### Nginx Configuration

```nginx
server {
    listen 80;
    server_name your-domain.com;

    # Redirect all HTTP to HTTPS
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name your-domain.com;

    # SSL Configuration
    ssl_certificate /path/to/your/certificate.crt;
    ssl_certificate_key /path/to/your/private.key;

    # Security headers
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header Referrer-Policy "no-referrer-when-downgrade" always;
    add_header Content-Security-Policy "default-src 'self' http: https: data: blob: 'unsafe-inline'" always;

    # Gzip compression
    gzip on;
    gzip_vary on;
    gzip_min_length 1024;
    gzip_proxied expired no-cache no-store private must-revalidate auth;
    gzip_types text/plain text/css text/xml text/javascript application/x-javascript application/xml+rss;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Security: Hide upstream information
    proxy_hide_header X-Powered-By;
}
```

### Apache Configuration

```apache
<VirtualHost *:80>
    ServerName your-domain.com
    Redirect permanent / https://your-domain.com/
</VirtualHost>

<VirtualHost *:443>
    ServerName your-domain.com

    # SSL Configuration
    SSLEngine on
    SSLCertificateFile /path/to/your/certificate.crt
    SSLCertificateKeyFile /path/to/your/private.key

    # Security headers
    Header always set X-Frame-Options "SAMEORIGIN"
    Header always set X-XSS-Protection "1; mode=block"
    Header always set X-Content-Type-Options "nosniff"
    Header always set Referrer-Policy "no-referrer-when-downgrade"

    ProxyPreserveHost On
    ProxyPass / http://127.0.0.1:8080/
    ProxyPassReverse / http://127.0.0.1:8080/

    # Security: Hide upstream information
    Header unset X-Powered-By
</VirtualHost>
```

## 🐳 Docker Deployment

### Dockerfile

```dockerfile
# Build stage
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y openssl ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy binary and assets
COPY --from=builder /app/target/release/dining-hall-dashboard .
COPY --from=builder /app/data ./data
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static
COPY --from=builder /app/assets ./assets

# Create user
RUN useradd -m -u 1000 app
USER app

# Expose port
EXPOSE 8080

# Run application
CMD ["./dining-hall-dashboard"]
```

### Docker Compose

```yaml
version: "3.8"

services:
  dining-hall-dashboard:
    build: .
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
      - SESSION_SECRET=your-64-character-secret-key-here
    volumes:
      - ./data:/app/data
    restart: unless-stopped
    security_opt:
      - no-new-privileges:true
```

### Building and Running

```bash
# Build the image
docker build -t dining-hall-dashboard .

# Run the container
docker run -d \
  --name dining-hall-dashboard \
  -p 8080:8080 \
  -e RUST_LOG=info \
  -e SESSION_SECRET=your-64-character-secret-key-here \
  -v $(pwd)/data:/app/data \
  --restart unless-stopped \
  dining-hall-dashboard
```

## 📊 Monitoring and Logging

### Application Logs

The application outputs logs to stdout/stderr. Configure your system's logging solution to capture these logs:

```bash
# Using systemd journal
journalctl -u dining-hall-dashboard -f

# Using log files
# Redirect output when starting the application
./dining-hall-dashboard > /var/log/dining-hall-dashboard/app.log 2>&1
```

### Health Checks

Implement health checks in your deployment:

```bash
# Simple health check
curl -f http://localhost:8080/health || exit 1

# More comprehensive health check
curl -f http://localhost:8080/api/items | grep -q "id" || exit 1
```

### Metrics and Monitoring

Consider implementing monitoring for:

1. **Application Uptime**
2. **Response Times**
3. **Error Rates**
4. **Resource Usage (CPU, Memory, Disk)**
5. **Request Volume**

## 🔁 Systemd Service (Linux)

Create a systemd service file for automatic startup:

```ini
# /etc/systemd/system/dining-hall-dashboard.service
[Unit]
Description=Dining Hall Dashboard
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/opt/dining-hall-dashboard
Environment=RUST_LOG=info
Environment=SESSION_SECRET=your-64-character-secret-key-here
ExecStart=/opt/dining-hall-dashboard/dining-hall-dashboard
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable dining-hall-dashboard
sudo systemctl start dining-hall-dashboard
sudo systemctl status dining-hall-dashboard
```

## 🔐 SSL/TLS Configuration

### Let's Encrypt with Certbot

1. **Install Certbot**

   ```bash
   sudo apt install certbot python3-certbot-nginx  # For Nginx
   # or
   sudo apt install certbot python3-certbot-apache  # For Apache
   ```

2. **Obtain Certificate**

   ```bash
   sudo certbot --nginx -d your-domain.com  # For Nginx
   # or
   sudo certbot --apache -d your-domain.com  # For Apache
   ```

3. **Auto-renewal**

   ```bash
   # Test renewal
   sudo certbot renew --dry-run

   # Add to crontab for automatic renewal
   echo "0 12 * * * /usr/bin/certbot renew --quiet" | sudo crontab -
   ```

## 🔧 Backup and Recovery

### Data Backup

Regularly backup the data directory:

```bash
# Create backup script
#!/bin/bash
BACKUP_DIR="/backup/dining-hall-dashboard"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p $BACKUP_DIR/$DATE
cp -r /etc/dining-hall-dashboard/data $BACKUP_DIR/$DATE/
tar -czf $BACKUP_DIR/dining-hall-dashboard-backup-$DATE.tar.gz -C $BACKUP_DIR $DATE
rm -rf $BACKUP_DIR/$DATE
```

### Recovery Procedure

1. **Stop the application**

   ```bash
   sudo systemctl stop dining-hall-dashboard
   ```

2. **Restore data**

   ```bash
   tar -xzf /backup/dining-hall-dashboard-backup-YYYYMMDD_HHMMSS.tar.gz -C /tmp
   cp -r /tmp/YYYYMMDD_HHMMSS/data/* /etc/dining-hall-dashboard/data/
   ```

3. **Restart the application**
   ```bash
   sudo systemctl start dining-hall-dashboard
   ```

## 🆘 Troubleshooting

### Common Issues

#### Application Won't Start

- Check file permissions on data directory
- Verify SESSION_SECRET is set
- Check logs for error messages

#### Permission Denied Errors

```bash
# Fix data directory permissions
sudo chown -R www-data:www-data /etc/dining-hall-dashboard/data
sudo chmod -R 755 /etc/dining-hall-dashboard/data
```

#### Port Already in Use

```bash
# Check what's using the port
sudo lsof -i :8080
# Kill the process
sudo kill -9 <PID>
```

### Log Analysis

Monitor logs for common issues:

```bash
# View recent errors
journalctl -u dining-hall-dashboard --since "1 hour ago" | grep -i error

# View access logs
journalctl -u dining-hall-dashboard --since "1 hour ago" | grep "GET\|POST"
```

## 📈 Performance Tuning

### Memory Usage

The application typically uses 50-100MB RAM under normal load. Monitor usage and adjust system resources as needed.

### Connection Handling

The Actix-web framework efficiently handles concurrent connections. For high-traffic deployments, consider:

1. **Load Balancing**: Use multiple instances behind a load balancer
2. **Caching**: Implement caching for static assets
3. **Compression**: Enable gzip compression in your reverse proxy

### File System

Ensure the data directory is on a reliable storage system with adequate I/O performance.

## 🔄 Updates and Maintenance

### Update Procedure

1. **Backup data**

   ```bash
   # Backup current data
   cp -r /etc/dining-hall-dashboard/data /backup/dining-hall-dashboard/data-$(date +%Y%m%d)
   ```

2. **Stop the service**

   ```bash
   sudo systemctl stop dining-hall-dashboard
   ```

3. **Update the application**

   ```bash
   # For binary deployment
   wget https://github.com/your-org/dining-hall-dashboard/releases/latest/download/dining-hall-dashboard.tar.gz
   tar -xzf dining-hall-dashboard.tar.gz
   sudo cp target/release/dining-hall-dashboard /opt/dining-hall-dashboard/
   ```

4. **Start the service**

   ```bash
   sudo systemctl start dining-hall-dashboard
   ```

5. **Verify the update**
   ```bash
   curl -f http://localhost:8080/health
   ```

## 📚 Additional Resources

- [Actix-web Documentation](https://actix.rs/docs/)
- [Rust Deployment Guide](https://doc.rust-lang.org/book/ch00-00-introduction.html)
- [Nginx Configuration Guide](https://nginx.org/en/docs/)
- [Apache Configuration Guide](https://httpd.apache.org/docs/)
- [Docker Documentation](https://docs.docker.com/)

## 📞 Support

For deployment issues or questions:

- Check the documentation
- File an issue on GitHub
- Contact charlie@charlimit.com
