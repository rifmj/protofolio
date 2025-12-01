# Security Policy

## Supported Versions

We currently support the following versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability, please report it responsibly:

1. **Do not** open a public issue
2. Email the maintainer directly at: **jum.rifm@gmail.com**
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will acknowledge receipt within 48 hours and provide a timeline for addressing the issue.

## Security Practices

This project follows security best practices:

- **Dependency Scanning**: We use `cargo-deny` to check for known vulnerabilities in dependencies
- **No Unsafe Code**: The codebase denies unsafe code at the workspace level
- **Regular Updates**: Dependencies are kept up to date
- **Security Audits**: Regular security audits are performed

### Running Security Checks

To check for security vulnerabilities in dependencies:

```bash
# Install cargo-deny if not already installed
cargo install cargo-deny

# Run security checks
cargo deny check
```

This will check for:
- Known vulnerabilities (CVEs)
- Unmaintained crates
- License compliance
- Banned dependencies

## Security Configuration

Security settings are configured in `deny.toml`. This file controls:
- Advisory database checks (vulnerabilities, unmaintained crates)
- License policies
- Banned dependencies
- Source verification

