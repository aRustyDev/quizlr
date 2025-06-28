# Installation

Quizlr is designed to be easy to install and use. This guide covers all installation options.

## Web Version (Recommended)

The easiest way to use Quizlr is through your web browser.

### Hosted Version

> ⚠️ **Coming Soon**: The hosted version is not yet available. For now, please use the self-hosted option.

Visit [quizlr.app](https://quizlr.app) to use the hosted version - no installation required!

### Self-Hosted

To run Quizlr on your own server:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/quizlr.git
   cd quizlr
   ```

2. **Install dependencies**:
   ```bash
   # Install Rust (if not already installed)
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Install Just (command runner)
   cargo install just
   
   # Install project dependencies
   just setup
   ```

3. **Build the application**:
   ```bash
   just build-web
   ```

4. **Serve the files**:
   ```bash
   # Using Python
   cd quizlr-web/dist
   python3 -m http.server 8080
   
   # Or using any static file server
   ```

5. **Open in browser**: Navigate to `http://localhost:8080`

## Desktop Application

> ⚠️ **Coming Soon**: Desktop applications for Windows, macOS, and Linux are planned for future releases.

## Mobile Applications

> ⚠️ **Coming Soon**: Native iOS and Android apps are in development.

## Browser Extension

> ⚠️ **Coming Soon**: Browser extensions for Chrome and Firefox are planned.

## System Requirements

### Minimum Requirements

- **Browser**: Chrome 90+, Firefox 88+, Safari 14+, Edge 90+
- **RAM**: 2GB
- **Storage**: 100MB for application, plus space for your data
- **Internet**: Required for AI features and sync

### Recommended Requirements

- **Browser**: Latest version of Chrome, Firefox, or Edge
- **RAM**: 4GB or more
- **Storage**: 1GB+ for comfortable usage
- **Internet**: Broadband connection for best performance

## Verifying Installation

After installation, verify everything is working:

1. Open Quizlr in your browser
2. Check for any error messages in the console (F12 → Console tab)
3. Try creating a simple quiz
4. Test taking the quiz

If you encounter issues, see the [Troubleshooting](./troubleshooting.md) guide.

## Next Steps

Now that Quizlr is installed, continue to:
- [Quick Start Guide](./getting-started.md) - Start using Quizlr
- [Configuration](./configuration.md) - Customize your setup
- [API Keys Setup](./api-keys.md) - Enable AI features