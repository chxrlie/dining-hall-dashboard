## 🔐 Security Features

The Dining Hall Dashboard implements multiple security measures to protect data and user sessions:

- **Argon2 Password Hashing**: Industry-standard password security with salt
- **Secure HTTP-only Cookies**: Session cookies with HttpOnly and Secure flags
- **CSRF Protection**: Cross-site request forgery protection tokens
- **Input Validation**: Server-side validation and sanitization of all inputs
- **Session Expiration**: Automatic session expiration after 24 hours
- **Rate Limiting**: Built-in rate limiting on authentication endpoints

## Cors Policy

The application uses a permissive CORS policy to allow requests from any origin. This is convenient for development, but it is recommended to restrict the policy to a specific domain in production.

## ⚙️ Configuration

The application can be configured using environment variables.

| Variable         | Description                                                                 | Default Value        |
| ---------------- | --------------------------------------------------------------------------- | -------------------- |
| `RUST_LOG`       | The log level for the application.                                          | `info`               |
| `SESSION_SECRET` | A secret key for encrypting session data. **Required for production.**      | A hardcoded dev key. |
| `PORT`           | The port on which the server will listen.                                   | `8080`               |
| `HOST`           | The host address to which the server will bind.                             | `0.0.0.0`            |

### Session Management

For development, a fixed session key is used to ensure session persistence across server restarts. For production deployments, it is **critical** to set a secure, randomly generated `SESSION_SECRET` to protect user sessions.