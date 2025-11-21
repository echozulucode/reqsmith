# AGENT.md - ReqSmith Project Guide for AI Assistants

**Project**: ReqSmith  
**Type**: Desktop Application (Tauri + Rust + React)  
**Purpose**: ReqIF requirements management editor with Apple-quality UX  
**Last Updated**: 2025-11-21

---

## Project Overview

ReqSmith is a modern desktop requirements management tool built with Rust and Tauri. It focuses on lossless ReqIF interoperability, keyboard-first navigation, and a clean, distraction-free user interface inspired by Apple design principles.

### Key Goals

1. **Full ReqIF compliance** - Load, edit, and save ReqIF 1.2 files without data loss
2. **Performance** - Handle 10,000+ requirements smoothly
3. **Superior UX** - Keyboard-driven, minimalist interface
4. **Interoperability** - Compatible with DOORS, Polarion, Jama, and other tools

---

## Project Structure

```
reqsmith/
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── main.rs         # Entry point
│   │   ├── commands.rs     # Tauri IPC commands
│   │   ├── reqif/          # ReqIF parsing and serialization
│   │   │   ├── model.rs    # Data structures
│   │   │   ├── parser.rs   # XML parsing
│   │   │   └── serializer.rs # XML writing
│   │   ├── search/         # Full-text search (Tantivy)
│   │   │   ├── index.rs
│   │   │   └── query.rs
│   │   ├── links.rs        # Traceability link management
│   │   ├── baseline.rs     # Version snapshots
│   │   ├── validation.rs   # Quality checks
│   │   ├── export.rs       # Export to Excel, CSV, PDF
│   │   └── undo.rs         # Command pattern for undo/redo
│   └── Cargo.toml
├── src/                    # React frontend
│   ├── App.tsx             # Main app component
│   ├── components/
│   │   ├── Layout.tsx      # Three-pane layout
│   │   ├── TreeView.tsx    # Specification tree
│   │   ├── RequirementsTable.tsx # Requirements list
│   │   ├── DetailPane.tsx  # Requirement details
│   │   ├── SearchBar.tsx   # Search interface
│   │   ├── InlineEdit.tsx  # Inline editing
│   │   ├── RichTextEditor.tsx # XHTML editor
│   │   ├── LinksPanel.tsx  # Traceability links
│   │   ├── GraphView.tsx   # Traceability graph
│   │   ├── DiffView.tsx    # Baseline comparison
│   │   ├── SavedViews.tsx  # Custom views
│   │   └── FileMenu.tsx    # File operations
│   └── package.json
├── docs/
│   ├── ai/plans/
│   │   ├── plan-001-high-level.md      # Design plan
│   │   └── plan-002-implementation.md  # Implementation roadmap
│   └── architecture/       # (Future) Architecture docs
├── tests/                  # Integration tests
│   └── fixtures/           # Sample ReqIF files
├── README.md
├── AGENT.md               # This file
└── LICENSE
```

---

## Technology Stack

### Backend (Rust)

- **Framework**: Tauri 2.x (desktop application framework)
- **XML Parsing**: `quick-xml` (performance), `yaserde` (serialization)
- **Search**: `tantivy` (full-text + faceted search)
- **Graph**: `petgraph` (traceability algorithms)
- **Serialization**: `serde`, `serde_json`
- **ZIP**: `zip` (for .reqifz files)
- **Async**: `tokio`
- **Error Handling**: `thiserror`, `anyhow`

### Frontend (React + TypeScript)

- **UI Framework**: React 18+ with TypeScript
- **State Management**: Zustand or Jotai (lightweight)
- **Styling**: TailwindCSS (utility-first)
- **Virtualization**: `react-window` (large lists)
- **Rich Text**: Lexical or TipTap (XHTML editing)
- **Graphs**: D3.js or Cytoscape.js (traceability visualization)
- **Testing**: Vitest, React Testing Library

### Build & Development

- **Package Manager**: pnpm (frontend), Cargo (backend)
- **Linting**: clippy (Rust), eslint + prettier (TypeScript)
- **CI/CD**: GitHub Actions
- **Testing**: cargo test, vitest

---

## Development Phases

The project follows a phased roadmap (see `docs/ai/plans/plan-002-implementation.md`):

1. **Phase 0**: Project setup (1 week)
2. **Phase 1**: MVP - ReqIF I/O (4 weeks)
3. **Phase 2**: Navigation and viewing (4 weeks)
4. **Phase 3**: Editing (3 weeks)
5. **Phase 4**: Search and filtering (3 weeks)
6. **Phase 5**: Traceability and links (3 weeks)
7. **Phase 6**: Advanced features (4 weeks)
8. **Phase 7**: Polish and performance (3 weeks)

**Total**: ~25 weeks (6 months)

---

## Key Concepts

### ReqIF (Requirements Interchange Format)

- **Standard**: OMG/XML format for requirements exchange
- **Version**: ReqIF 1.2
- **Core Elements**:
  - `SpecObject`: Individual requirements with attributes
  - `SpecRelation`: Links between requirements
  - `Specification`: Hierarchical structure
  - `DatatypeDefinition`: Attribute types (Boolean, Integer, Real, String, Enumeration, XHTML)
- **Files**: `.reqif` (XML) or `.reqifz` (ZIP with attachments)

### Data Flow

1. **Load**: Parse ReqIF XML → Build in-memory model → Index for search
2. **Edit**: User changes → Update model → Mark dirty → Optional auto-save
3. **Save**: Serialize model → Write XML → Preserve unknown elements (round-trip)
4. **Search**: User query → Tantivy search → Return results → Display in UI

### Architecture Patterns

- **Backend**: Command pattern (undo/redo), builder pattern (queries)
- **Frontend**: Component composition, custom hooks for state
- **IPC**: Tauri commands for backend communication
- **Error Handling**: `Result<T, E>` in Rust, exceptions in TypeScript

---

## Coding Standards

### Rust

- **Style**: Follow `rustfmt` defaults
- **Linting**: All clippy warnings must pass
- **Errors**: Use `thiserror` for custom errors, `anyhow` for application errors
- **Testing**: Unit tests alongside implementation, integration tests in `tests/`
- **Documentation**: Public APIs must have doc comments
- **Naming**: Snake_case for functions/variables, PascalCase for types

### TypeScript/React

- **Style**: Prettier with 2-space indentation
- **Components**: Functional components with hooks
- **Props**: Define explicit types/interfaces
- **State**: Prefer local state, lift when shared
- **Naming**: camelCase for functions/variables, PascalCase for components
- **Files**: One component per file, co-locate tests

### Git Workflow

- **Branches**: `main` (stable), `develop` (integration), feature branches
- **Commits**: Conventional commits (feat:, fix:, docs:, etc.)
- **PRs**: Required for main, descriptive titles, linked issues

---

## Common Tasks

### Setting Up Development Environment

```bash
# Clone repository
git clone <repo-url>
cd reqsmith

# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js dependencies (frontend)
cd reqsmith
pnpm install

# Install Tauri CLI
cargo install tauri-cli

# Run development server
cargo tauri dev
```

### Building for Production

```bash
# Build optimized binary
cargo tauri build

# Output in src-tauri/target/release/
```

### Running Tests

```bash
# Rust tests
cd src-tauri
cargo test

# Frontend tests
cd ..
pnpm test

# Integration tests
cargo test --test '*'
```

### Adding a New Tauri Command

1. Define function in `src-tauri/src/commands.rs`:
   ```rust
   #[tauri::command]
   pub fn my_command(param: String) -> Result<ReturnType, String> {
       // Implementation
   }
   ```

2. Register in `src-tauri/src/main.rs`:
   ```rust
   tauri::Builder::default()
       .invoke_handler(tauri::generate_handler![my_command])
   ```

3. Call from frontend:
   ```typescript
   import { invoke } from '@tauri-apps/api/tauri';
   const result = await invoke<ReturnType>('my_command', { param: 'value' });
   ```

### Adding a New React Component

1. Create file: `src/components/MyComponent.tsx`
2. Define component:
   ```typescript
   interface MyComponentProps {
       prop1: string;
   }
   
   export function MyComponent({ prop1 }: MyComponentProps) {
       return <div>{prop1}</div>;
   }
   ```
3. Add tests: `src/components/MyComponent.test.tsx`

---

## Important Files and Patterns

### ReqIF Model (`src-tauri/src/reqif/model.rs`)

Core data structures mirroring ReqIF schema. Key types:

```rust
pub struct ReqIF {
    pub header: ReqIFHeader,
    pub core_content: CoreContent,
    pub tool_extensions: Vec<ToolExtension>,
}

pub struct SpecObject {
    pub identifier: String,
    pub spec_type: SpecTypeRef,
    pub values: Vec<AttributeValue>,
}

pub enum AttributeValue {
    Boolean(bool),
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
    XHTML(String),
}
```

### Parser Strategy

- Use `quick-xml::Reader` for streaming parse
- Build index: `HashMap<String, SpecObject>` for O(1) lookups
- Preserve unknown XML elements in `extra_attrs: HashMap<String, String>`
- Validate IDs and references during parse

### Search Architecture

- **Indexing**: On file load, index all text fields + facets
- **Query**: Support full-text + structured filters
- **Updates**: Incremental index updates on edit
- **Performance**: <100ms query response for 10K requirements

### UI Layout

Three-pane design (resizable):

```
+----------------+---------------------------+------------------+
|                |                           |                  |
|  Spec Tree     |   Requirements Table      |  Detail Pane     |
|  (TreeView)    |   (RequirementsTable)     |  (DetailPane)    |
|                |                           |                  |
|  - Spec 1      |   ID | Title | Status     |  Attributes:     |
|    - Spec 1.1  |   001 | Req 1 | Draft     |   ID: 001        |
|    - Spec 1.2  |   002 | Req 2 | Approved  |   Title: ...     |
|  - Spec 2      |   003 | Req 3 | Review    |   Description:   |
|                |                           |   [XHTML]        |
+----------------+---------------------------+------------------+
```

---

## Design Principles

### Apple-Era UX Guidelines

1. **Focus and Deference**: UI should not distract from content
2. **Hierarchy and Typography**: Clear visual structure, system fonts
3. **Motion and Feedback**: Subtle, meaningful animations (no gratuitous effects)
4. **Progressive Disclosure**: Hide complexity, reveal on demand
5. **Consistency**: Predictable patterns throughout

### Keyboard-First Design

- All major actions accessible via keyboard
- Shortcuts follow platform conventions (Cmd on Mac, Ctrl on Windows)
- Vim-like navigation optional (j/k for up/down)
- Tab order logical and predictable

### Performance Targets

- Load 10K requirements: <2 seconds
- Search query: <100ms
- UI render: 60fps (smooth scrolling)
- Save file: <1 second for typical files

---

## Testing Strategy

### Unit Tests

- **Rust**: Test each module independently
  - ReqIF parser: Test with sample XML files
  - Search: Test indexing and queries
  - Links: Test graph operations
- **React**: Test components in isolation with React Testing Library

### Integration Tests

- End-to-end Tauri command tests
- Test workflows: Load → Edit → Save → Reload
- Round-trip tests: parse(serialize(X)) === X

### Manual QA

- Test with real ReqIF files from DOORS, Polarion, Jama
- Test on all platforms: Windows, macOS, Linux
- Accessibility testing: Keyboard nav, screen readers

### Test Data

- Sample files in `tests/fixtures/`:
  - `small.reqif`: 10 requirements (quick tests)
  - `medium.reqif`: 1000 requirements (performance baseline)
  - `large.reqif`: 10000 requirements (stress test)
  - `real-world-*.reqif`: Exported from actual tools

---

## Common Issues and Solutions

### XML Parsing Errors

**Problem**: Invalid XML or malformed ReqIF files  
**Solution**: Graceful error handling, report line number and context

### Performance with Large Files

**Problem**: Slow loading or UI lag  
**Solution**: Streaming parse, lazy rendering (react-window), Tantivy indexing

### Round-Trip Data Loss

**Problem**: Unknown elements not preserved  
**Solution**: Store unknown XML in `extra_attrs`, serialize back unchanged

### XHTML Rendering

**Problem**: Security risks (XSS), formatting issues  
**Solution**: Sanitize HTML (DOMPurify), use safe subset of XHTML

### Cross-Platform Issues

**Problem**: Keyboard shortcuts differ (Cmd vs Ctrl)  
**Solution**: Use Tauri's platform detection, map shortcuts dynamically

---

## Dependencies and Licenses

### Key Crates

- `tauri`: MIT/Apache-2.0
- `quick-xml`: MIT
- `tantivy`: MIT
- `serde`: MIT/Apache-2.0
- `petgraph`: MIT/Apache-2.0

### Frontend Libraries

- `react`: MIT
- `typescript`: Apache-2.0
- `tailwindcss`: MIT

**Project License**: TBD (likely MIT or Apache-2.0 for open source)

---

## Roadmap and Current Status

### Current Phase: Phase 0 (Project Setup)

**Completed**:
- [x] High-level design document
- [x] Implementation plan
- [x] AGENT.md documentation

**Next Steps**:
1. Initialize Tauri project
2. Set up project structure
3. Configure build system and CI/CD
4. Create sample ReqIF test files
5. Begin Phase 1: ReqIF data model

### Future Milestones

- **MVP (Phase 1-2)**: Load/view/save ReqIF files (Week 9)
- **Editing (Phase 3)**: In-place editing (Week 12)
- **Search (Phase 4)**: Full-text search (Week 15)
- **Traceability (Phase 5)**: Links and graph (Week 18)
- **Advanced (Phase 6)**: Baselines, validation, export (Week 22)
- **Release (Phase 7)**: Polish and 1.0 release (Week 25)

---

## Resources and References

### ReqIF Specification

- **Official Spec**: [OMG ReqIF 1.2](https://www.omg.org/spec/ReqIF/1.2/)
- **Schema**: XSD files for validation
- **Tutorial**: ReqIF Implementor Forum resources

### Tauri Documentation

- **Guides**: https://tauri.app/v1/guides/
- **API Reference**: https://tauri.app/v1/api/
- **Examples**: https://github.com/tauri-apps/tauri/tree/dev/examples

### Related Projects

- **ReqView**: Commercial ReqIF editor (Windows)
- **ProR**: Eclipse-based requirements tool
- **RMSIS**: Requirements management plugin

### Design Inspiration

- **Things**: Task manager with clean UI
- **Notion**: Flexible workspace design
- **Obsidian**: Knowledge base with graph view
- **Linear**: Issue tracker with keyboard-first UX

---

## Working with AI Assistants

### Effective Prompts

**Good**:
- "Implement the ReqIF parser for SpecObject elements following the structure in model.rs"
- "Add a search bar component with keyboard shortcut Cmd/Ctrl+F"
- "Write unit tests for the link index with bidirectional lookup"

**Avoid**:
- "Make it better" (too vague)
- "Add all features" (scope creep)
- "Fix the bug" (no context)

### Context to Provide

When asking for help, include:
1. **File paths**: Which files are involved
2. **Current behavior**: What happens now
3. **Desired behavior**: What should happen
4. **Constraints**: Performance, compatibility requirements
5. **Related code**: Relevant structs, functions, or components

### Generated Code Review

Always review AI-generated code for:
- **Correctness**: Does it match the spec?
- **Safety**: Proper error handling, no panics
- **Performance**: Appropriate data structures and algorithms
- **Style**: Follows project conventions
- **Tests**: Adequate test coverage

---

## Contribution Guidelines

### Before Starting

1. Read the design and implementation plans
2. Check existing issues and PRs
3. Discuss major changes in an issue first

### Pull Request Process

1. Create feature branch from `develop`
2. Write code + tests
3. Run linters and tests locally
4. Write clear commit messages
5. Submit PR with description and screenshots (if UI)
6. Address review feedback

### Code Review Checklist

- [ ] Follows coding standards
- [ ] Includes tests
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
- [ ] Passes CI checks

---

## Contact and Support

- **GitHub Issues**: Bug reports and feature requests
- **Discussions**: Design questions and ideas
- **Email**: (TBD)

---

## Changelog

- **2025-11-21**: Initial AGENT.md created
  - Project structure defined
  - Development phases outlined
  - Coding standards established
  - Testing strategy documented

---

**End of AGENT.md**

*This document is maintained alongside the codebase and should be updated as the project evolves. When in doubt, refer to the implementation plan and high-level design in `docs/ai/plans/`.*
