# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

We take the security of Quizlr seriously. If you have discovered a security vulnerability, please follow these steps:

### 1. Do NOT Create a Public Issue

Security vulnerabilities should be reported privately to avoid putting users at risk.

### 2. Email Us Directly

Send details to: security@quizlr.app (or your actual security email)

Include:
- Type of vulnerability
- Affected components
- Steps to reproduce
- Potential impact
- Any suggested fixes

### 3. Wait for Initial Response

We will acknowledge receipt within 48 hours and provide an expected timeline for a fix.

### 4. Coordinate Disclosure

We will work with you to understand the issue and develop a fix. We request that you:
- Give us reasonable time to fix the issue before public disclosure
- Avoid exploiting the vulnerability beyond what's necessary for reporting
- Help us understand the impact

## Security Best Practices for Users

### API Keys
- Never commit API keys to version control
- Use environment variables or secure key management
- Rotate keys regularly
- Use separate keys for development and production

### Authentication
- Use strong passwords
- Enable two-factor authentication when available
- Regularly review authorized applications

### Data Storage
- Encrypt sensitive data at rest
- Use secure connections (HTTPS)
- Regular backups of important data
- Review sharing permissions

## Security Features

Quizlr implements several security measures:

- **Input Validation**: All user inputs are validated and sanitized
- **CORS Protection**: Proper CORS headers for web security
- **Content Security Policy**: CSP headers to prevent XSS attacks
- **Secure Storage**: Sensitive data is encrypted
- **Authentication**: OAuth 2.0/OIDC support with major providers
- **Rate Limiting**: API rate limiting to prevent abuse
- **Audit Logging**: Security-relevant events are logged

## Dependencies

We regularly update dependencies to patch known vulnerabilities:
- Automated dependency updates via Dependabot
- Security audit in CI/CD pipeline
- Manual review of security advisories

## Incident Response

In case of a security incident:

1. **Immediate Mitigation**: Deploy fixes for critical vulnerabilities
2. **User Notification**: Inform affected users promptly
3. **Post-Mortem**: Analyze and document lessons learned
4. **Improvements**: Update security measures based on findings

## Recognition

We appreciate security researchers who help keep Quizlr secure. Contributors will be acknowledged (with permission) in our security hall of fame.

Thank you for helping keep Quizlr and its users safe!