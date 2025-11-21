# ReqSmith Implementation Plan

**Version**: 1.0  
**Date**: 2025-11-21  
**Based on**: plan-001-high-level.md

---

## 1. Overview

This implementation plan breaks down the high-level design into concrete development phases, modules, tasks, and deliverables. The goal is to build a Rust+Tauri desktop ReqIF editor with Apple-quality UX that surpasses existing requirements management tools.

### 1.1 Tech Stack Summary

- **Backend**: Rust (core logic, ReqIF parsing, search indexing)
- **Frontend**: React + TypeScript
- **Desktop Framework**: Tauri 2.x
- **Search**: Tantivy (full-text + faceted search)
- **XML Parsing**: quick-xml + yaserde
- **Persistence**: In-memory (MVP), then SQLite or sled
- **Graph**: petgraph (traceability visualization)

### 1.2 Success Criteria

- Load, view, edit, and save ReqIF 1.2 files with zero data loss
- Handle files with 10,000+ requirements efficiently
- Keyboard-first navigation and editing
- Clean, distraction-free UI following Apple design principles

---

## 2. Development Phases

### Phase 0: Project Setup (1 week)

**Goal**: Establish project structure, tooling, and build pipeline.

**Tasks**:
1. Initialize Tauri project with Rust backend and React frontend
2. Set up project structure:
   - `src-tauri/` (Rust backend)
   - `src/` (React frontend)
   - `docs/` (documentation)
   - `tests/` (integration tests)
3. Configure build system (Cargo.toml, package.json)
4. Set up linting (clippy, eslint, prettier)
5. Configure CI/CD (GitHub Actions for build + test)
6. Create sample ReqIF test files (small, medium, large)

**Deliverables**:
- Empty Tauri app that launches
- README with build instructions
- CI pipeline running basic checks

---

### Phase 1: MVP - Core ReqIF I/O (3-4 weeks)

**Goal**: Load and save ReqIF files with full fidelity.

#### 1.1 ReqIF Data Model (1 week)

**Module**: `src-tauri/src/reqif/model.rs`

**Tasks**:
1. Define Rust structs for ReqIF core types:
   - `ReqIF` (root)
   - `SpecObject` (requirements)
   - `SpecRelation` (links)
   - `Specification` (hierarchy)
   - `SpecType`, `AttributeDefinition`, `AttributeValue`
   - `DatatypeDefinition` (Boolean, Integer, Real, String, Enumeration, XHTML)
2. Implement serde serialization/deserialization
3. Support for `.reqif` (XML) and `.reqifz` (ZIP)
4. Handle namespaces and preserve unknown XML elements
5. Unit tests with sample ReqIF files

**Key Decisions**:
- Use `quick-xml` for parsing (performance)
- Use `yaserde` for struct mapping
- Store unknown XML nodes in `HashMap<String, String>` for round-trip

**Deliverables**:
- Rust structs modeling ReqIF 1.2 schema
- Parse/serialize functions
- Tests validating round-trip integrity

#### 1.2 ReqIF Parser (1 week)

**Module**: `src-tauri/src/reqif/parser.rs`

**Tasks**:
1. Implement streaming XML parser with `quick-xml`
2. Build in-memory index of SpecObjects by ID
3. Parse embedded XHTML (store as string, sanitize on display)
4. Handle malformed files gracefully (report warnings)
5. Extract attachments from `.reqifz`
6. Validate against ReqIF XSD schema (optional)

**Error Handling**:
- Return `Result<ReqIF, ReqIFError>` with detailed error messages
- Log warnings for non-critical issues (missing optional fields)

**Deliverables**:
- Parser that loads ReqIF into Rust model
- Error handling for invalid files
- Tests with real-world ReqIF exports

#### 1.3 ReqIF Serializer (1 week)

**Module**: `src-tauri/src/reqif/serializer.rs`

**Tasks**:
1. Serialize Rust model back to XML
2. Preserve original formatting (indentation, namespace prefixes)
3. Write `.reqifz` files (ZIP with manifest)
4. Ensure round-trip: parse(serialize(X)) == X

**Deliverables**:
- Serializer outputting valid ReqIF XML
- Round-trip tests

#### 1.4 Tauri Commands for File Operations (0.5 week)

**Module**: `src-tauri/src/commands.rs`

**Tasks**:
1. `open_reqif(path: String) -> Result<ReqIF, String>`
2. `save_reqif(path: String, data: ReqIF) -> Result<(), String>`
3. Integrate with Tauri file dialog

**Deliverables**:
- Tauri commands exposed to frontend
- File picker integration

#### 1.5 Basic UI: Load and Save (0.5 week)

**Module**: `src/App.tsx`, `src/components/FileMenu.tsx`

**Tasks**:
1. Menu: File → Open, Save, Save As
2. Display success/error notifications
3. Show loading spinner during parse

**Deliverables**:
- UI to open/save ReqIF files
- Basic error display

---

### Phase 2: Navigation and Viewing (3-4 weeks)

**Goal**: Display requirements in tree, list, and detail views.

#### 2.1 Core UI Layout (1 week)

**Module**: `src/components/Layout.tsx`

**Tasks**:
1. Three-pane layout:
   - Left: Specification tree (collapsible)
   - Center: Requirements table/list
   - Right: Detail pane (read-only for now)
2. Resizable panes with drag handles
3. Keyboard shortcuts (Cmd/Ctrl+O, Cmd/Ctrl+S, Cmd/Ctrl+F)

**Design**:
- Minimal chrome, generous whitespace
- System fonts (San Francisco on Mac, Segoe UI on Windows)
- Subtle shadows for depth hierarchy

**Deliverables**:
- Three-pane layout component
- Responsive resize behavior

#### 2.2 Specification Tree View (1 week)

**Module**: `src/components/TreeView.tsx`

**Tasks**:
1. Render hierarchical tree from `Specification`
2. Expand/collapse nodes
3. Click to select requirement
4. Show icons for requirement type
5. Keyboard navigation (arrow keys, Enter to expand)

**Tauri Command**:
- `get_specification_tree() -> SpecificationNode`

**Deliverables**:
- Interactive tree view
- Navigation with keyboard

#### 2.3 Requirements Table (1 week)

**Module**: `src/components/RequirementsTable.tsx`

**Tasks**:
1. Display selected specification's requirements in table
2. Columns: ID, Title, Status, Priority (configurable)
3. Sorting by column
4. Row selection (single/multi)
5. Virtualized scrolling for large lists (react-window)

**Tauri Command**:
- `get_requirements(spec_id: String) -> Vec<SpecObject>`

**Deliverables**:
- Performant table view
- Column sorting and configuration

#### 2.4 Detail Pane (1 week)

**Module**: `src/components/DetailPane.tsx`

**Tasks**:
1. Display selected requirement's attributes
2. Render XHTML content (sanitized)
3. Show embedded images
4. Display links (SpecRelations)
5. Read-only view for Phase 2

**Tauri Command**:
- `get_requirement_detail(id: String) -> SpecObject`

**Deliverables**:
- Rich detail view
- XHTML rendering

---

### Phase 3: Editing (2-3 weeks)

**Goal**: Enable in-place editing of requirements.

#### 3.1 Inline Editing (1 week)

**Module**: `src/components/InlineEdit.tsx`

**Tasks**:
1. Double-click or F2 to edit cell in table
2. Tab/Shift+Tab to move between fields
3. Esc to cancel, Enter to save
4. Validation for required fields
5. Optimistic UI updates

**Tauri Command**:
- `update_attribute(req_id: String, attr: String, value: AttributeValue) -> Result<(), String>`

**Deliverables**:
- Keyboard-driven editing
- Validation feedback

#### 3.2 Rich Text Editor for XHTML (1.5 weeks)

**Module**: `src/components/RichTextEditor.tsx`

**Tasks**:
1. Integrate editor (Lexical, TipTap, or ProseMirror)
2. Support bold, italic, lists, tables
3. Paste from Word/Excel with cleanup
4. Serialize back to ReqIF XHTML format
5. Image embedding

**Tauri Command**:
- `update_xhtml(req_id: String, html: String) -> Result<(), String>`

**Deliverables**:
- WYSIWYG editor for requirement text
- Clean XHTML output

#### 3.3 Undo/Redo (0.5 week)

**Module**: `src-tauri/src/undo.rs`

**Tasks**:
1. Command pattern for all mutations
2. Undo/redo stack (limit to 100 actions)
3. Keyboard shortcuts (Cmd/Ctrl+Z, Cmd/Ctrl+Shift+Z)

**Deliverables**:
- Reliable undo/redo

---

### Phase 4: Search and Filtering (2-3 weeks)

**Goal**: Fast, powerful search and filtering.

#### 4.1 Tantivy Integration (1 week)

**Module**: `src-tauri/src/search/index.rs`

**Tasks**:
1. Index SpecObjects on load
2. Index fields: ID, title, description, all text attributes
3. Facets for enumerations (status, priority, type)
4. Incremental updates on edit
5. Persist index to disk (optional)

**Deliverables**:
- Tantivy index for full-text search
- Faceted search support

#### 4.2 Search UI (1 week)

**Module**: `src/components/SearchBar.tsx`

**Tasks**:
1. Search bar at top (Cmd/Ctrl+F to focus)
2. Live search results as you type
3. Highlight matches in results
4. Advanced search modal (filters by attribute)
5. Recent searches dropdown

**Tauri Command**:
- `search(query: String, filters: HashMap<String, String>) -> Vec<SpecObject>`

**Deliverables**:
- Fast search bar
- Filter UI

#### 4.3 Saved Views and Filters (1 week)

**Module**: `src/components/SavedViews.tsx`

**Tasks**:
1. Create and save custom views (filter + sort + columns)
2. Quick access sidebar for views
3. Preset views: "All Requirements", "Untested", "High Priority"
4. Export view configuration (JSON)

**Deliverables**:
- Custom view management
- Preset views

---

### Phase 5: Traceability and Links (2-3 weeks)

**Goal**: Visualize and manage requirement relationships.

#### 5.1 Link Management (1 week)

**Module**: `src-tauri/src/links.rs`

**Tasks**:
1. Parse SpecRelations on load
2. Build bi-directional index (forward + backlinks)
3. Support for custom link types
4. CRUD operations for links

**Tauri Commands**:
- `get_links(req_id: String) -> Vec<Link>`
- `create_link(source: String, target: String, type: String) -> Result<(), String>`
- `delete_link(link_id: String) -> Result<(), String>`

**Deliverables**:
- Link index and management

#### 5.2 Traceability UI (1 week)

**Module**: `src/components/LinksPanel.tsx`

**Tasks**:
1. Show related requirements in detail pane
2. Click to navigate to linked requirement
3. Visual indicators (upstream/downstream)
4. Drag-and-drop to create links

**Deliverables**:
- Links displayed in UI
- Easy link creation

#### 5.3 Graph View (1 week)

**Module**: `src/components/GraphView.tsx`

**Tasks**:
1. Render traceability graph with D3.js or Cytoscape.js
2. Zoom/pan/filter controls
3. Highlight paths between selected requirements
4. Export graph as image (SVG/PNG)

**Backend**:
- Use `petgraph` for graph algorithms (shortest path, cycle detection)

**Tauri Command**:
- `get_graph_data(root_id: String, depth: u32) -> GraphData`

**Deliverables**:
- Interactive traceability graph

---

### Phase 6: Advanced Features (3-4 weeks)

**Goal**: Baselining, diff, validation, export.

#### 6.1 Baselines (1 week)

**Module**: `src-tauri/src/baseline.rs`

**Tasks**:
1. Snapshot current state with metadata (name, date, user)
2. Store baselines as separate ReqIF files or internal snapshots
3. UI to create/view baselines

**Deliverables**:
- Baseline creation and storage

#### 6.2 Diff View (1 week)

**Module**: `src/components/DiffView.tsx`

**Tasks**:
1. Compare current state to baseline
2. Show added/modified/deleted requirements
3. Attribute-level diffs
4. Side-by-side or unified view

**Tauri Command**:
- `diff(baseline_id: String) -> DiffResult`

**Deliverables**:
- Visual diff interface

#### 6.3 Validation and Quality Checks (1 week)

**Module**: `src-tauri/src/validation.rs`

**Tasks**:
1. Configurable rules: required attributes, link constraints
2. Run validation on demand or save
3. Display issues in UI (filterable list)
4. Quick-fix actions

**Rules**:
- Required fields not empty
- No orphaned requirements
- All high-priority reqs have tests

**Deliverables**:
- Validation engine
- Issues panel

#### 6.4 Export and Reporting (1 week)

**Module**: `src-tauri/src/export.rs`

**Tasks**:
1. Export to Excel (.xlsx)
2. Export to CSV
3. Export to PDF (via HTML template + headless browser)
4. Custom report templates

**Deliverables**:
- Multi-format export

---

### Phase 7: Polish and Performance (2-3 weeks)

**Goal**: Optimize, test, and refine UX.

#### 7.1 Performance Optimization (1 week)

**Tasks**:
1. Profile parsing and search with large files (10K+ reqs)
2. Lazy-load detail pane content
3. Optimize Tantivy indexing
4. Reduce memory footprint (use Rc/Arc strategically)
5. Async file I/O

**Targets**:
- Load 10K requirements in <2s
- Search query response <100ms
- Smooth 60fps UI

**Deliverables**:
- Benchmarks and optimizations

#### 7.2 UX Polish (1 week)

**Tasks**:
1. Animations and transitions (subtle, Apple-style)
2. Empty states with helpful prompts
3. Onboarding tour for new users
4. Context menus (right-click)
5. Tooltips and help text
6. Dark mode support

**Deliverables**:
- Polished UI with attention to detail

#### 7.3 Testing (1 week)

**Tasks**:
1. Unit tests for all Rust modules (80%+ coverage)
2. Integration tests for Tauri commands
3. UI tests with React Testing Library
4. Manual QA with real-world ReqIF files from DOORS, Polarion, Jama
5. Accessibility audit (keyboard nav, screen readers)

**Deliverables**:
- Comprehensive test suite
- QA report

---

## 3. Module Breakdown

### 3.1 Rust Backend Modules

| Module | Description | Priority |
|--------|-------------|----------|
| `reqif/model.rs` | Core data structures | P0 |
| `reqif/parser.rs` | XML parsing | P0 |
| `reqif/serializer.rs` | XML writing | P0 |
| `search/index.rs` | Tantivy indexing | P1 |
| `search/query.rs` | Search execution | P1 |
| `links.rs` | Link management | P2 |
| `baseline.rs` | Baseline snapshots | P3 |
| `validation.rs` | Quality checks | P3 |
| `export.rs` | Multi-format export | P3 |
| `undo.rs` | Command pattern | P2 |
| `commands.rs` | Tauri IPC | P0 |

### 3.2 React Frontend Components

| Component | Description | Priority |
|-----------|-------------|----------|
| `App.tsx` | Main app shell | P0 |
| `Layout.tsx` | Three-pane layout | P0 |
| `TreeView.tsx` | Specification tree | P0 |
| `RequirementsTable.tsx` | Table view | P0 |
| `DetailPane.tsx` | Detail view | P0 |
| `SearchBar.tsx` | Search interface | P1 |
| `InlineEdit.tsx` | Inline editing | P1 |
| `RichTextEditor.tsx` | XHTML editor | P1 |
| `LinksPanel.tsx` | Links display | P2 |
| `GraphView.tsx` | Traceability graph | P2 |
| `DiffView.tsx` | Baseline comparison | P3 |
| `SavedViews.tsx` | View management | P2 |
| `FileMenu.tsx` | File operations | P0 |

---

## 4. Risks and Mitigation

### 4.1 Technical Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| ReqIF parsing complexity | High | Use battle-tested libraries; test with real exports |
| Performance with large files | High | Streaming parse, lazy load, Tantivy indexing |
| Round-trip data loss | High | Comprehensive round-trip tests; preserve unknown XML |
| Tauri/WebView limitations | Medium | Prototype early; fallback to native dialogs |
| XHTML editor complexity | Medium | Use proven editor library (Lexical); limit features |

### 4.2 UX Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Too much complexity | High | Progressive disclosure; hide advanced features |
| Poor keyboard nav | Medium | Keyboard-first design from day 1 |
| Graph clutter | Medium | Smart filtering; zoom controls; limit default depth |

### 4.3 Schedule Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Scope creep | High | Stick to phased roadmap; defer non-MVP features |
| Library incompatibilities | Medium | Evaluate libraries early; have backup options |
| Testing gaps | Medium | Test continuously; allocate dedicated QA time |

---

## 5. Dependencies and Tools

### 5.1 Rust Crates

- `tauri` - Desktop framework
- `quick-xml` - XML parsing
- `yaserde` - XML serialization
- `serde`, `serde_json` - Data serialization
- `tantivy` - Full-text search
- `petgraph` - Graph algorithms
- `zip` - .reqifz handling
- `thiserror` - Error handling
- `tokio` - Async runtime
- `rayon` - Parallelism (optional)

### 5.2 Frontend Libraries

- `react`, `react-dom`
- `typescript`
- `@tauri-apps/api` - Tauri bindings
- `react-window` - Virtualized lists
- `lexical` or `tiptap` - Rich text editor
- `d3` or `cytoscape` - Graph visualization
- `zustand` or `jotai` - State management
- `react-query` - Async state
- `tailwindcss` - Styling

### 5.3 Dev Tools

- `clippy`, `rustfmt` - Rust linting
- `eslint`, `prettier` - JS linting
- `vitest` - Frontend tests
- `cargo-tarpaulin` - Coverage
- GitHub Actions - CI/CD

---

## 6. Success Metrics

### 6.1 Feature Completeness

- [ ] Load and save ReqIF without data loss
- [ ] Display 10K+ requirements smoothly
- [ ] Full-text search <100ms
- [ ] Keyboard-driven editing
- [ ] Traceability links and graph
- [ ] Baselines and diff

### 6.2 Quality Metrics

- **Test Coverage**: >80% for Rust, >70% for frontend
- **Performance**: Load 10K reqs in <2s
- **Accessibility**: WCAG 2.1 AA compliant
- **Error Rate**: <1% crashes/failures in QA

### 6.3 User Validation

- Beta test with 5-10 users
- Positive feedback on UX simplicity
- Successful import of real-world files from 3+ tools

---

## 7. Timeline Summary

| Phase | Duration | Cumulative |
|-------|----------|------------|
| Phase 0: Setup | 1 week | 1 week |
| Phase 1: MVP I/O | 4 weeks | 5 weeks |
| Phase 2: Navigation | 4 weeks | 9 weeks |
| Phase 3: Editing | 3 weeks | 12 weeks |
| Phase 4: Search | 3 weeks | 15 weeks |
| Phase 5: Traceability | 3 weeks | 18 weeks |
| Phase 6: Advanced | 4 weeks | 22 weeks |
| Phase 7: Polish | 3 weeks | **25 weeks** |

**Total**: ~6 months (single developer, full-time)

Adjust for team size, part-time work, or scope changes.

---

## 8. Next Steps

1. **Immediate** (Week 1):
   - Set up Tauri project
   - Initialize Git repo with basic structure
   - Create sample ReqIF test files
   - Write detailed module specs

2. **Short-term** (Weeks 2-5):
   - Implement ReqIF parser
   - Build basic UI layout
   - First working demo: open file → display tree

3. **Medium-term** (Weeks 6-15):
   - Complete MVP (view + edit)
   - Implement search
   - Beta release to early users

4. **Long-term** (Weeks 16-25):
   - Advanced features (links, baselines, export)
   - Polish and performance
   - Public release

---

## 9. Open Questions

1. **Persistence**: Keep everything in memory or use SQLite for large files?
   - **Decision**: Start in-memory; add optional SQLite backend in Phase 6.

2. **Collaboration**: Multi-user editing or single-user only?
   - **Decision**: Single-user for MVP; defer collaboration to post-1.0.

3. **Licensing**: Open source or proprietary?
   - **Decision**: TBD (consider open core model).

4. **Platform Support**: Windows, Mac, Linux?
   - **Decision**: All three (Tauri supports all).

5. **Mobile**: Support mobile devices?
   - **Decision**: Desktop-only for 1.0; mobile is out of scope.

---

## 10. Conclusion

This implementation plan provides a concrete roadmap from empty project to fully-featured ReqIF editor. The phased approach ensures steady progress with testable milestones. By focusing on Apple-quality UX and leveraging modern Rust tooling, ReqSmith will deliver a superior requirements management experience.

**Next**: Begin Phase 0 setup and validate assumptions with early prototypes.
