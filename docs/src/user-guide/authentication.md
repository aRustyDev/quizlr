# Authentication

This guide covers authentication and security features in Quizlr to protect your data and control access.

## Authentication Overview

Quizlr provides multiple authentication methods:

```
Authentication Methods
├─ Local Authentication
│  ├─ No Authentication (Default)
│  ├─ Password Protection
│  └─ Biometric (Touch ID/Face ID)
├─ OAuth Providers
│  ├─ GitHub
│  ├─ Google
│  └─ Microsoft
└─ Enterprise Options
   ├─ SAML 2.0
   ├─ LDAP/Active Directory
   └─ Custom SSO
```

## Local Authentication

### Password Protection

Enable password protection for local access:

1. Go to **Settings** → **Security**
2. Click **Enable Password Protection**
3. Set a strong password:
   ```
   Password Requirements:
   ✓ At least 12 characters
   ✓ Mix of uppercase and lowercase
   ✓ Include numbers
   ✓ Include special characters
   
   Password Strength: ████████░░ Strong
   ```

4. Set security options:
   - Auto-lock after inactivity
   - Failed attempt limits
   - Password reset options

### Biometric Authentication

On supported devices:

**macOS (Touch ID):**
```yaml
authentication:
  biometric:
    enabled: true
    fallback_to_password: true
    require_password_after_days: 7
```

**Windows (Windows Hello):**
```yaml
authentication:
  windows_hello:
    enabled: true
    pin_fallback: true
    facial_recognition: true
    fingerprint: true
```

### Security Questions

Set up recovery options:

```
Security Questions (choose 3):
1. What was your first pet's name?
   Answer: [**********]
   
2. In what city were you born?
   Answer: [**********]
   
3. What is your mother's maiden name?
   Answer: [**********]

[✓] Answers are case-insensitive
[✓] Store encrypted recovery key
```

## OAuth Authentication

### GitHub Authentication

1. **Create GitHub OAuth App**:
   - Go to GitHub Settings → Developer settings → OAuth Apps
   - Click "New OAuth App"
   - Fill in details:
     ```
     Application name: Quizlr
     Homepage URL: https://quizlr.app
     Authorization callback URL: http://localhost:3000/auth/github/callback
     ```
   - Save Client ID and Secret

2. **Configure in Quizlr**:
   ```yaml
   oauth:
     github:
       enabled: true
       client_id: "${GITHUB_CLIENT_ID}"
       client_secret: "${GITHUB_CLIENT_SECRET}"
       scopes: ["read:user", "repo"]
       organizations: ["my-org"]  # Optional: restrict to org members
   ```

3. **User Experience**:
   ```
   Sign in with GitHub
   ├─ Click "Sign in with GitHub"
   ├─ Authorize Quizlr
   ├─ Grant permissions
   └─ Redirected back to Quizlr
   ```

### Google Authentication

1. **Set up Google OAuth**:
   - Visit [Google Cloud Console](https://console.cloud.google.com)
   - Create new project or select existing
   - Enable Google+ API
   - Create OAuth 2.0 credentials

2. **Configure credentials**:
   ```
   Authorized JavaScript origins:
   - http://localhost:3000
   - https://quizlr.app
   
   Authorized redirect URIs:
   - http://localhost:3000/auth/google/callback
   - https://quizlr.app/auth/google/callback
   ```

3. **Quizlr configuration**:
   ```yaml
   oauth:
     google:
       enabled: true
       client_id: "${GOOGLE_CLIENT_ID}"
       client_secret: "${GOOGLE_CLIENT_SECRET}"
       hosted_domain: "company.com"  # Optional: restrict to domain
   ```

### Microsoft Authentication

For Microsoft/Office 365 accounts:

```yaml
oauth:
  microsoft:
    enabled: true
    client_id: "${AZURE_CLIENT_ID}"
    client_secret: "${AZURE_CLIENT_SECRET}"
    tenant: "common"  # or specific tenant ID
    scopes: ["User.Read", "Files.Read"]
```

## Multi-Factor Authentication (MFA)

### Time-based OTP (TOTP)

Enable 2FA with authenticator apps:

1. Go to **Settings** → **Security** → **Two-Factor Authentication**
2. Click **Enable 2FA**
3. Scan QR code with authenticator app:
   ```
   ┌─────────────────┐
   │ █▀▀▀█ ▄▀█ █▀▀▀█ │
   │ █   █ ▀▄▀ █   █ │  Scan with:
   │ ▀▀▀▀▀ █▄█ ▀▀▀▀▀ │  • Google Authenticator
   │ ▀█▄▀█▄█▀▄██▀█▄▀ │  • Authy
   │ ▄▀▀▄▀ ▄ ▀▄█▄▀▀▄ │  • 1Password
   │ █▀▀▀█ ▀██▀█   █ │  • Microsoft Authenticator
   │ ▀▀▀▀▀ ▀▀▀▀▀▀▀▀▀ │
   └─────────────────┘
   
   Manual entry: JBSWY3DPEHPK3PXP
   ```

4. Enter verification code
5. Save backup codes safely

### Backup Codes

Generate and store recovery codes:

```
Recovery Codes (save these securely):
┌─────────────────────────────┐
│ 1. a4k9-3md8-9sk2-1pd9     │
│ 2. b9m3-4kd7-2ls9-4md8     │
│ 3. c2l8-9md3-4ks8-2pd1     │
│ 4. d8k2-1md9-3ls4-9pd8     │
│ 5. e3m9-4kd2-8ls1-3pd4     │
│ 6. f9l3-2md8-4ks9-1pd2     │
│ 7. g4k8-3md1-9ls2-4pd9     │
│ 8. h2m4-9kd8-3ls1-2pd4     │
└─────────────────────────────┘

[Download] [Print] [Copy to Clipboard]
```

### Hardware Keys

Support for FIDO2/WebAuthn:

```yaml
mfa:
  hardware_keys:
    enabled: true
    providers: ["yubikey", "titan", "feitian"]
    require_pin: true
    attestation: "direct"
```

## Session Management

### Session Configuration

Control session behavior:

```yaml
sessions:
  # Session Lifetime
  timeout_minutes: 30
  remember_me_days: 30
  
  # Security Options
  regenerate_id: true
  same_site: "strict"
  secure_cookie: true
  http_only: true
  
  # Device Management
  max_devices: 5
  device_fingerprinting: true
  notify_new_device: true
```

### Active Sessions

View and manage active sessions:

```
Active Sessions
├─ Current Session
│  ├─ Device: MacBook Pro (This device)
│  ├─ Location: San Francisco, CA
│  ├─ IP: 192.168.1.100
│  └─ Last active: Just now
├─ iPhone
│  ├─ Location: San Francisco, CA
│  ├─ IP: 192.168.1.101
│  └─ Last active: 2 hours ago
└─ iPad
   ├─ Location: New York, NY
   ├─ IP: 74.125.x.x
   └─ Last active: 3 days ago
   
[Revoke All Other Sessions]
```

### Session Security

Protect against session hijacking:

```yaml
security:
  session_protection:
    # IP Binding
    bind_to_ip: false  # May cause issues with mobile
    
    # User Agent Checking
    check_user_agent: true
    
    # Concurrent Session Limits
    concurrent_sessions: 3
    action_on_limit: "logout_oldest"  # or "prevent_new"
```

## Access Control

### Role-Based Access

Define roles and permissions:

```yaml
roles:
  admin:
    permissions:
      - create_quiz
      - edit_any_quiz
      - delete_quiz
      - manage_users
      - view_analytics
      
  teacher:
    permissions:
      - create_quiz
      - edit_own_quiz
      - view_student_results
      - export_data
      
  student:
    permissions:
      - take_quiz
      - view_own_results
      - create_practice_quiz
```

### Content Access Control

Control who can access quizzes:

```yaml
content_access:
  default_visibility: "private"  # private, unlisted, public
  
  sharing_options:
    - link_sharing
    - email_invitation
    - organization_only
    
  permissions:
    can_copy: false
    can_export: false
    require_authentication: true
```

### API Access

Manage API tokens:

```
API Tokens
├─ Personal Access Token
│  ├─ Name: Mobile App
│  ├─ Created: 2024-01-15
│  ├─ Last used: Today
│  ├─ Scopes: [read, write]
│  └─ [Revoke]
├─ Integration Token
│  ├─ Name: GitHub Actions
│  ├─ Created: 2024-01-01
│  ├─ Last used: Yesterday
│  ├─ Scopes: [read]
│  └─ [Revoke]

[Generate New Token]
```

## Enterprise Authentication

### SAML 2.0

Configure SAML for enterprise SSO:

```yaml
saml:
  enabled: true
  
  # Identity Provider Settings
  idp:
    entity_id: "https://idp.company.com"
    sso_url: "https://idp.company.com/sso"
    slo_url: "https://idp.company.com/slo"
    certificate: "${SAML_CERT}"
    
  # Service Provider Settings
  sp:
    entity_id: "https://quizlr.company.com"
    acs_url: "https://quizlr.company.com/saml/acs"
    
  # Attribute Mapping
  attributes:
    email: "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/emailaddress"
    name: "http://schemas.xmlsoap.org/ws/2005/05/identity/claims/name"
    groups: "http://schemas.xmlsoap.org/claims/Group"
```

### LDAP/Active Directory

Connect to corporate directory:

```yaml
ldap:
  enabled: true
  
  # Connection Settings
  server: "ldap://dc.company.com:389"
  use_tls: true
  bind_dn: "CN=Quizlr,OU=ServiceAccounts,DC=company,DC=com"
  bind_password: "${LDAP_PASSWORD}"
  
  # User Search
  user_base: "OU=Users,DC=company,DC=com"
  user_filter: "(&(objectClass=user)(sAMAccountName={username}))"
  
  # Group Mapping
  group_base: "OU=Groups,DC=company,DC=com"
  group_filter: "(member={dn})"
  admin_groups: ["CN=QuizlrAdmins,OU=Groups,DC=company,DC=com"]
```

## Security Features

### Audit Logging

Track authentication events:

```yaml
audit:
  enabled: true
  
  events:
    - login_success
    - login_failure
    - logout
    - password_change
    - mfa_enabled
    - session_revoked
    - permission_changed
    
  retention_days: 90
  
  export:
    format: "json"
    destination: "s3://audit-logs/quizlr/"
```

### Security Headers

Protect against common attacks:

```yaml
security_headers:
  # Prevent clickjacking
  x_frame_options: "DENY"
  
  # XSS Protection
  x_content_type_options: "nosniff"
  x_xss_protection: "1; mode=block"
  
  # Content Security Policy
  content_security_policy: |
    default-src 'self';
    script-src 'self' 'unsafe-inline';
    style-src 'self' 'unsafe-inline';
    img-src 'self' data: https:;
    
  # HSTS
  strict_transport_security: "max-age=31536000; includeSubDomains"
```

### Rate Limiting

Prevent brute force attacks:

```yaml
rate_limiting:
  login_attempts:
    max_attempts: 5
    window_minutes: 15
    lockout_minutes: 30
    
  api_requests:
    authenticated: 1000  # per hour
    unauthenticated: 100 # per hour
    
  password_reset:
    max_requests: 3
    window_hours: 24
```

## Privacy Settings

### Data Protection

Configure privacy options:

```yaml
privacy:
  # Data Retention
  retention:
    quiz_results_days: 365
    user_activity_days: 90
    deleted_data_days: 30
    
  # Anonymization
  anonymize:
    after_days: 180
    fields: ["ip_address", "user_agent", "location"]
    
  # Export Rights
  gdpr:
    allow_export: true
    allow_deletion: true
    deletion_cooldown_days: 30
```

### Cookie Settings

Manage cookie consent:

```yaml
cookies:
  # Essential Cookies (always enabled)
  essential:
    - session_id
    - csrf_token
    - authentication
    
  # Optional Cookies
  analytics:
    enabled: false
    provider: "none"
    
  preferences:
    enabled: true
    items: ["theme", "language", "display_settings"]
```

## Troubleshooting

### Common Issues

**Login Failed:**
```
Error: Invalid username or password
Solutions:
1. Check caps lock
2. Reset password
3. Clear browser cache
4. Try different browser
```

**MFA Issues:**
```
Error: Invalid authentication code
Solutions:
1. Check device time sync
2. Use backup code
3. Re-scan QR code
4. Contact administrator
```

**Session Expired:**
```
Error: Your session has expired
Solutions:
1. Log in again
2. Enable "Remember me"
3. Check session timeout settings
4. Disable aggressive browser privacy
```

### Debug Authentication

Enable auth debugging:

```yaml
debug:
  authentication:
    log_level: "debug"
    log_auth_attempts: true
    log_session_events: true
    mask_sensitive_data: true
```

### Recovery Options

Account recovery process:

1. **Self-Service Recovery**:
   - Click "Forgot Password"
   - Enter email or username
   - Answer security questions
   - Reset via email link

2. **Admin Recovery**:
   - Contact administrator
   - Verify identity
   - Admin generates reset link
   - Temporary password provided

3. **Emergency Access**:
   ```bash
   # Generate emergency access token
   quizlr admin recovery --user john@example.com
   
   # Reset user password
   quizlr admin reset-password --user john@example.com
   ```

## Best Practices

### Password Security

1. **Use a password manager**
2. **Enable MFA** on all accounts
3. **Unique passwords** for each service
4. **Regular password rotation**
5. **Avoid password patterns**

### Session Security

1. **Log out** when finished
2. **Don't share** session links
3. **Use secure networks**
4. **Keep software updated**
5. **Monitor active sessions**

### Administrative Security

1. **Limit admin accounts**
2. **Use separate admin credentials**
3. **Enable audit logging**
4. **Regular security reviews**
5. **Test recovery procedures**

## Next Steps

- [Configure Data Sources](./data-sources.md) for content
- [Set Up Data Management](./data-management.md)
- [Review Security Configuration](./configuration.md#security)
- [Troubleshooting Guide](./troubleshooting.md)

## Related Documentation

- [API Keys Security](./api-keys.md#security)
- [Configuration Guide](./configuration.md)
- [Privacy Policy](../legal/privacy.md)
- [Security Best Practices](../security/best-practices.md)