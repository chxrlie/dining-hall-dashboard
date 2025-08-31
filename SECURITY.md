# Dining Hall Dashboard Security Policy

## 🛡️ Overview

The Dining Hall Dashboard takes security seriously. This document outlines our security practices, policies, and procedures for reporting vulnerabilities.

## 🔒 Security Features

### Authentication Security

- **Password Hashing**: All passwords are hashed using Argon2, a memory-hard hashing function that provides resistance against brute-force and side-channel attacks
- **Session Management**: Secure HTTP-only cookies with configurable expiration times
- **CSRF Protection**: Cross-site request forgery protection tokens for all state-changing operations
- **Rate Limiting**: Built-in rate limiting on authentication endpoints to prevent brute force attacks

### Data Security

- **File-based Storage**: JSON files are stored with appropriate file permissions
- **Input Validation**: Server-side validation of all inputs to prevent injection attacks
- **Data Sanitization**: User-provided content is sanitized before storage and display

### Network Security

- **HTTPS Support**: Full HTTPS support for production deployments
- **CORS Configuration**: Controlled cross-origin resource sharing policies
- **Secure Headers**: Implementation of security headers to prevent common web vulnerabilities

### Access Control

- **Role-based Access**: Different levels of access for administrators and public users
- **Session Isolation**: User sessions are isolated to prevent cross-session data access

## 📋 Security Best Practices

### For Developers

1. **Input Validation**: Always validate and sanitize user inputs
2. **Dependency Management**: Regularly update dependencies to address known vulnerabilities
3. **Secure Coding**: Follow secure coding practices and conduct code reviews
4. **Error Handling**: Never expose sensitive information in error messages
5. **Logging**: Implement appropriate logging without storing sensitive data

### For Administrators

1. **Password Policies**: Enforce strong password policies for admin accounts
2. **Regular Updates**: Keep the application and its dependencies up to date
3. **Backup Strategy**: Implement regular backups of data files
4. **Access Control**: Limit administrative access to authorized personnel only
5. **Monitoring**: Monitor logs for suspicious activities

### For Deployment

1. **Environment Variables**: Use environment variables for sensitive configuration
2. **File Permissions**: Set appropriate file permissions for data files
3. **Network Security**: Use firewalls and network segmentation where appropriate
4. **HTTPS**: Always use HTTPS in production environments
5. **Session Security**: Configure secure session settings for production

## 🔐 Password Security

### Hashing Algorithm

We use Argon2, the winner of the Password Hashing Competition, which provides:

- Resistance to GPU-based attacks
- Adjustable memory and CPU requirements
- Protection against side-channel attacks

### Implementation Details

```rust
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::HashError)?
        .to_string();
    Ok(password_hash)
}
```

### Password Requirements

While the application doesn't enforce specific password requirements, we recommend:

- Minimum 12 characters
- Mix of uppercase, lowercase, numbers, and special characters
- No dictionary words or personal information
- Regular password changes

## 🍪 Session Security

### Cookie Settings

- **HttpOnly**: Prevents client-side JavaScript access to cookies
- **Secure**: Ensures cookies are only sent over HTTPS
- **SameSite**: Controls when cookies are sent with cross-site requests
- **Expiration**: Sessions automatically expire after 24 hours of inactivity

### Session Management

- Sessions are renewed on successful login
- Sessions are purged on logout
- Session data is stored securely

## 🛑 Rate Limiting

### Implementation

Rate limiting is implemented to prevent abuse:

- **Authentication**: 10 requests per minute per IP address
- **API Endpoints**: 100 requests per minute per authenticated session

### Configuration

Rate limiting parameters can be adjusted based on deployment requirements.

## 🐞 Vulnerability Reporting

### Reporting a Vulnerability

We appreciate responsible disclosure of security vulnerabilities. If you discover a security issue, please follow these steps:

1. **Do not** create a public GitHub issue
2. **Email** security@your-org.com with the following information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Your contact information (optional but preferred)

### Response Process

1. **Acknowledgment**: We will acknowledge your report within 48 hours
2. **Investigation**: Our security team will investigate the issue
3. **Resolution**: We will work on a fix and timeline for resolution
4. **Update**: We will keep you informed of our progress
5. **Disclosure**: We will coordinate public disclosure once the issue is resolved

### Scope

This policy applies to the latest stable release of the Dining Hall Dashboard. We do not provide bug bounties but will acknowledge valid reports in our release notes.

## 🔍 Security Audits

### Regular Audits

We conduct regular security reviews of:

- Dependencies for known vulnerabilities
- Code for security best practices
- Configuration for secure deployment

### Third-party Audits

We welcome third-party security audits and assessments. Please contact our security team to coordinate.

## 📚 Additional Resources

### Security Libraries

- [Argon2](https://github.com/RustCrypto/password-hashes/tree/master/argon2)
- [Actix-web Security](https://actix.rs/docs/security/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)

### Compliance

While this is a small application, we follow principles from:

- OWASP Secure Coding Practices
- NIST Cybersecurity Framework
- ISO 27001 Information Security Management

## 🔄 Version History

### v0.1.3 (Current)

- Updated Argon2 implementation
- Enhanced session security
- Improved input validation

### v0.1.2

- Added rate limiting
- Fixed CSRF protection implementation
- Updated dependency versions

### v0.1.1

- Initial security implementation
- Basic authentication system
- File permission guidelines

## 📞 Contact

For security-related questions or concerns:

- **Email**: charlie@charlimit.com
- **PGP Key**: [Available upon request]

---

_This security policy is subject to change as we improve our security practices and respond to new threats._
