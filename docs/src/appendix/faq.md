# Frequently Asked Questions (FAQ)

## General Questions

### What is Quizlr?

Quizlr is an open-source, adaptive quiz platform that combines traditional quiz functionality with modern AI capabilities. It's designed to make learning more engaging and effective through intelligent question generation, adaptive difficulty, and comprehensive analytics.

### Who is Quizlr for?

Quizlr is designed for:
- **Educators**: Create and manage quizzes for students
- **Students**: Practice and learn through adaptive quizzes
- **Trainers**: Build assessment tools for professional development
- **Content Creators**: Develop interactive learning experiences
- **Developers**: Extend and customize the platform

### Is Quizlr free to use?

Yes! Quizlr is open-source software released under the MIT License. You can:
- Use it for free forever
- Modify the source code
- Deploy your own instance
- Contribute to development
- Build commercial applications

### What makes Quizlr different from other quiz platforms?

Key differentiators:
- **Open Source**: Full transparency and community-driven
- **AI-Powered**: LLM integration for intelligent features
- **Adaptive Learning**: Adjusts to user performance
- **Developer-Friendly**: Extensive APIs and plugin system
- **Privacy-Focused**: Self-hostable with data ownership

## Technical Questions

### What technologies does Quizlr use?

- **Backend**: Rust (performance and safety)
- **Frontend**: Leptos (reactive web framework)
- **Styling**: Tailwind CSS
- **Database**: Flexible storage API (SQLite default)
- **Build**: WebAssembly for browser deployment
- **AI**: Multiple LLM provider support

### What are the system requirements?

#### For Users (Web App)
- Modern web browser (Chrome, Firefox, Safari, Edge)
- JavaScript enabled
- 2GB RAM minimum
- Internet connection (for cloud features)

#### For Self-Hosting
- 4GB RAM minimum
- 2 CPU cores
- 10GB storage
- Linux, macOS, or Windows
- Rust 1.70+ (for building from source)

### Can I use Quizlr offline?

Yes, with limitations:
- Local storage mode works offline
- PWA support enables offline access
- AI features require internet connection
- Sync happens when reconnected

### Does Quizlr work on mobile devices?

- **Web App**: Yes, fully responsive design
- **Native Apps**: Coming in v0.5.0 (Q1 2025)
- **PWA**: Installable on mobile devices
- **Touch Support**: Optimized for touch interfaces

## Feature Questions

### What types of questions does Quizlr support?

1. **True/False**: Simple binary choices
2. **Multiple Choice**: Single correct answer
3. **Multi-Select**: Multiple correct answers
4. **Fill in the Blank**: Text completion
5. **Match Pairs**: Connect related items
6. **Interactive Interview**: Conversational format
7. **Topic Explanation**: Long-form responses

### How does the AI integration work?

The AI features include:
- **Question Generation**: Create questions from topics
- **Answer Evaluation**: Smart grading for open-ended questions
- **Content Suggestions**: Recommend related topics
- **Adaptive Difficulty**: Adjust based on performance
- **Natural Language**: Conversational interactions

### Can I import existing quizzes?

Yes! Supported formats:
- JSON (native format)
- CSV (basic structure)
- Markdown (with frontmatter)
- QTI (coming soon)
- Moodle XML (planned)

### How does scoring work?

Four scoring strategies available:
1. **Simple**: Basic correct/incorrect
2. **Time-Weighted**: Faster responses score higher
3. **Difficulty-Weighted**: Harder questions worth more
4. **Adaptive**: Personalized based on performance

## Privacy & Security Questions

### How is my data stored?

- **Local Mode**: All data on your device
- **Self-Hosted**: Your server, your control
- **Cloud Mode**: Encrypted in transit and at rest
- **No Tracking**: No analytics without consent
- **Data Export**: Download all your data anytime

### Is my data shared with third parties?

- **Never** sold or shared for marketing
- **Optional** AI providers for LLM features
- **Transparent** about all data processing
- **Configurable** privacy settings
- **GDPR Compliant** design

### Can I delete my data?

Yes, completely:
- Delete individual items anytime
- Full account deletion available
- Data export before deletion
- No retention after deletion
- Clear documentation of the process

## Development Questions

### How can I contribute to Quizlr?

Many ways to help:
1. **Code**: Fix bugs, add features
2. **Documentation**: Improve guides
3. **Translation**: Localize the app
4. **Testing**: Report bugs, test features
5. **Design**: UI/UX improvements
6. **Ideas**: Suggest enhancements

### What's the development setup process?

Quick start:
```bash
# Clone repository
git clone https://github.com/yourusername/quizlr.git

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and run
cd quizlr
cargo run
```

See the [Development Setup Guide](../developer-guide/development-setup.md) for details.

### Can I create plugins or extensions?

Yes! Extension system supports:
- Custom question types
- New scoring strategies
- Storage backends
- UI themes
- API integrations

### How do I report bugs?

1. Check existing issues on GitHub
2. Create detailed bug report
3. Include reproduction steps
4. Attach relevant logs
5. Tag with appropriate labels

## Usage Questions

### How many quizzes can I create?

- **No artificial limits** on quiz creation
- **Storage based** on your plan/server
- **Performance tested** up to 10,000 quizzes
- **Bulk operations** supported
- **Archive feature** for old quizzes

### Can multiple people take the same quiz?

Yes, with features for:
- Unique session per user
- Randomized question order
- Time limits per session
- Leaderboards (optional)
- Anonymous mode available

### How do I share quizzes?

Multiple sharing options:
- Direct link sharing
- Embed in websites
- QR codes
- Email invitations
- API access

### Can I customize the appearance?

Extensive customization:
- Theme selection
- Custom CSS
- Logo/branding
- Color schemes
- Font choices

## Troubleshooting Questions

### Why isn't my quiz loading?

Common causes:
1. Browser cache (clear it)
2. Network issues (check connection)
3. JavaScript disabled (enable it)
4. Outdated browser (update it)
5. Server issues (check status)

### How do I recover lost work?

Built-in protections:
- Auto-save every 30 seconds
- Draft recovery system
- Version history
- Undo/redo support
- Manual save option

### Why are AI features not working?

Check these:
1. API key configured correctly
2. Internet connection active
3. API limits not exceeded
4. Provider service status
5. Correct model selected

### How do I get help?

Support channels:
- GitHub Issues (bugs/features)
- Discord Community (chat)
- Documentation (self-help)
- Stack Overflow (tagged 'quizlr')
- Email support (commercial users)

## Commercial Questions

### Can I use Quizlr for my business?

Yes! The MIT License allows:
- Commercial use
- Private modifications
- Paid services
- White-labeling
- No royalties

### Is there a hosted version?

- **Community Cloud**: Free tier planned
- **Pro Cloud**: Paid features (coming)
- **Enterprise**: Custom deployment
- **Always Free**: Self-hosting option

### Do you offer support contracts?

For enterprise users:
- Priority support
- SLA guarantees
- Custom development
- Training services
- Deployment assistance

### Can I sponsor development?

Yes! Support options:
- GitHub Sponsors
- Open Collective
- Corporate sponsorship
- Feature bounties
- Development partnerships

## Future Questions

### What's coming next?

See our [Roadmap](roadmap.md) for detailed plans:
- Mobile apps (Q1 2025)
- Advanced AI (Q3 2024)
- Collaboration (Q4 2024)
- Enterprise features (2025)

### Will Quizlr always be open source?

**Yes!** Core commitment:
- MIT License forever
- Community-driven development
- Transparent roadmap
- No vendor lock-in
- Fork-friendly approach

### How can I stay updated?

Follow progress:
- GitHub Watch/Star
- Release notifications
- Blog/Newsletter
- Discord announcements
- Twitter updates

---

**Can't find your answer?** 

- Search our [documentation](../introduction.md)
- Ask in [Discord](https://discord.gg/quizlr)
- Open a [GitHub issue](https://github.com/yourusername/quizlr/issues)
- Email us at support@quizlr.dev

*Last updated: December 2023*