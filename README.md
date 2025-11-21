# ReqSmith

**A modern desktop requirements management tool built with Rust and Tauri**

ReqSmith is a ReqIF editor that focuses on lossless interoperability, keyboard-first navigation, and a clean, distraction-free user interface inspired by Apple design principles.

## Features (Planned)

- ✅ Full ReqIF 1.2 compliance - Load, edit, and save without data loss
- ✅ High performance - Handle 10,000+ requirements smoothly
- ✅ Keyboard-first navigation and editing
- ✅ Full-text search with Tantivy
- ✅ Traceability links and graph visualization
- ✅ Baselines and change tracking
- ✅ Cross-platform (Windows, macOS, Linux)

## Tech Stack

- **Backend**: Rust (ReqIF parsing, search, business logic)
- **Frontend**: React + TypeScript
- **Desktop Framework**: Tauri 2.x
- **Search**: Tantivy
- **XML**: quick-xml + yaserde
- **Styling**: TailwindCSS

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) 1.70+
- [Node.js](https://nodejs.org/) 18+ and pnpm
- Platform-specific dependencies for Tauri (see [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/))

### Installation

```bash
# Clone the repository
git clone <repo-url>
cd reqsmith

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

### Building

```bash
# Build for production
pnpm tauri build

# Output will be in src-tauri/target/release/
```

## Project Structure

```
reqsmith/
├── src/                    # React frontend
│   ├── components/         # UI components
│   └── App.tsx            # Main app component
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── reqif/         # ReqIF parsing and serialization
│   │   ├── search/        # Full-text search (Tantivy)
│   │   └── commands.rs    # Tauri IPC commands
│   └── Cargo.toml
├── docs/                  # Documentation
│   └── ai/plans/         # Design and implementation plans
├── tests/                 # Integration tests
│   └── fixtures/         # Sample ReqIF files
├── AGENT.md              # AI assistant guide
└── README.md
```

## Development

### Running Tests

```bash
# Rust tests
cd src-tauri
cargo test

# Frontend tests (when added)
pnpm test
```

### Code Quality

```bash
# Rust linting
cd src-tauri
cargo clippy

# Format Rust code
cargo fmt

# TypeScript linting (when configured)
pnpm lint
```

## Documentation

- [AGENT.md](./AGENT.md) - Comprehensive guide for developers and AI assistants
- [High-Level Design](./docs/ai/plans/plan-001-high-level.md)
- [Implementation Plan](./docs/ai/plans/plan-002-implementation.md)

## Development Roadmap

See [Implementation Plan](./docs/ai/plans/plan-002-implementation.md) for detailed phases:

- **Phase 0**: Project setup ✅ (Current)
- **Phase 1**: MVP - Core ReqIF I/O (4 weeks)
- **Phase 2**: Navigation and viewing (4 weeks)
- **Phase 3**: Editing (3 weeks)
- **Phase 4**: Search and filtering (3 weeks)
- **Phase 5**: Traceability and links (3 weeks)
- **Phase 6**: Advanced features (4 weeks)
- **Phase 7**: Polish and performance (3 weeks)

## Contributing

Contributions are welcome! Please read [AGENT.md](./AGENT.md) for development guidelines and project context.

## License

TBD (likely MIT or Apache-2.0)

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/)
  - [Tauri Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode)
  - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
  - [ESLint](https://marketplace.visualstudio.com/items?itemName=dbaeumer.vscode-eslint)
  - [Prettier](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)
