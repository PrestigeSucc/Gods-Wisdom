# gw


  This tool is being built for people who enjoy reading/studying the Lord's Word, for people who enjoy cli/tui tools, and for 
people who enjoy speed and efficiency!


A command-line Bible tool, written in Rust.

> ✍️ **TODO:** replace this section with your own description of what the
> project actually does and why it exists.

---

## Status

🚧 Early days -- phase 1 of the roadmap below. The core lookup engine
compiles, runs, and has passing tests, but verse text is still a
placeholder (no real KJV/ASV/WEB data wired up yet). We are still working on
learning rust aswell, so technically still in phase 0, but ye.

## Layout

This is a Cargo **workspace** made of three crates:

| Crate | Purpose |
|---|---|
| `gw-core` | The library. All real logic lives here: parsing references, looking up verses, searching. No `main.rs` -- it's not a program, just code the other crates call into. |
| `gw-cli` | The single-shot command-line binary (`gw get "John 3:16"`, etc). Thin wrapper: parses args with `clap`, calls into `gw-core`, prints the result. |
| `gw-tui` | The full-screen interactive mode. Currently a stub -- real work starts in phase 3 of the roadmap. |

Splitting it this way means the TUI can reuse all the same lookup/search
logic as the CLI later, instead of duplicating it.

<details>
<summary><strong>Why a workspace instead of one crate?</strong> (click to expand)</summary>

<br>

Keeping the actual logic (`gw-core`) separate from the interfaces that use
it (`gw-cli`, `gw-tui`) means:

- The CLI and TUI can never drift out of sync, since they call the exact
  same functions.
- `gw-core` can be unit tested on its own, without needing a terminal or
  any argument parsing involved.
- Adding a future interface (a web API? a Discord bot?) would mean adding
  another thin crate, not rewriting any lookup logic.

</details>

## Running it

From the workspace root:

```sh
# Build everything
cargo build

# Run the CLI directly
cargo run --bin gw -- get "John 3:16"

# Run the TUI (currently just a placeholder message)
cargo run --bin gw-tui

# Run gw-core's unit tests
cargo test
```

## Roadmap

A living checklist of where this project is headed. Check items off as
they're finished -- GitHub renders `- [x]` as a ticked checkbox. (btw the check boxes are for when both of complete said thing or
when it is genuinely done -Jober)

### Phase 0 -- Rust ramp-up (learning)
> Get comfortable with the language before building on it for real.

- [ ] Work through *The Rust Book*, chapters 1-10 (ownership, structs, enums, error handling)
- [ ] Each build one tiny throwaway CLI (e.g. a word counter) using `clap`
- [ ] Learn cargo basics: workspaces, adding deps, `cargo check` / `clippy` / `fmt`
- [ ] Pair on a tiny shared toy project to practice git workflow + code review together
- [ ] Read `ratatui`'s official tutorial and run their counter-app example locally

### Phase 1 -- Foundation: core lookup engine 
> Ship a working binary that can look up any verse or passage offline, fast.

- [x] Project scaffolding: Cargo workspace split into `gw-core` (lib), `gw-cli` (bin), `gw-tui` (placeholder)
- [ ] Bundle KJV, ASV, WEB as embedded or locally-cached JSON/SQLite data
- [x] Reference parser: `"John 3:16"`, handles multi-word books like `"1 Corinthians 13:4"`
- [x] Core command: `gw get`
- [ ] Core commands: `gw search`, `gw random`
- [ ] CI pipeline: GitHub Actions, `cargo test`, cross-platform builds (Linux/macOS/Windows)

### Phase 2 -- Search & scripting 
> Make the tool genuinely useful in pipelines and for fast reference work.

- [ ] Full-text search across translations with relevance ranking
- [ ] Structured output modes: `--json`, `--plain`, `--color` for scripting
- [ ] Exit codes and stdin/stdout piping support (unix-friendly)
- [ ] Shell completions (`clap_complete`) for bash/zsh/fish
- [ ] Config file support (default translation, color preferences)
- [ ] Refactor: confirm all lookup/search logic lives in `gw-core`, callable from CLI or TUI

### Phase 3 -- TUI mode: full-screen interactive UI 
> Build the terminal UI as its own crate consuming `gw-core` -- a real second interface, not an afterthought.

- [ ] Learn `ratatui` basics: `terminal.draw` loop, widgets, layout constraints (small toy app first)
- [ ] App architecture: central state struct, key event handling, render function
- [ ] Core TUI views: browse by book/chapter, search results list, reading pane
- [ ] Navigation: vim-style keybindings (`j`/`k`/`g`/`G`) plus arrow keys, help overlay
- [ ] Bookmarks/notes panel inside the TUI, backed by local SQLite
- [ ] Launch TUI as default when run with no args; single-shot commands still work as before
- [ ] Polish: scrolling, multi-pane layout, resize handling

### Phase 4 -- Study & polish 
> Add study depth, then focus hard on packaging and distribution.

- [ ] Cross-reference lookups (e.g. Treasury of Scripture Knowledge data) in both CLI and TUI
- [ ] Side-by-side translation comparison view
- [ ] Reading plans with local progress tracking
- [ ] Publish to crates.io, Homebrew tap, AUR, winget
- [ ] Man pages, full `--help` text, documentation site (mdBook)
- [ ] Performance pass: startup time, binary size, memory use
- [ ] Community contribution guide; tag a 1.0 release

## Tech notes

<details>
<summary>Click to expand current dependency choices and why</summary>

<br>

- **`clap`** (derive API) -- argument parsing for `gw-cli`. Standard choice
  for Rust CLIs; the derive macros keep argument definitions close to the
  struct they populate.
- **`anyhow`** -- used in the binaries (`gw-cli`, `gw-tui`) where errors
  just need to be reported, not matched on.
- **`thiserror`** -- used in `gw-core`, the library. Lets callers match on
  specific error variants (`InvalidReference`, `UnknownBook`,
  `VerseOutOfRange`) rather than getting one generic error type.
- **`serde` / `serde_json`** -- for whatever data format the bundled
  Bible text ends up in.
- **`ratatui` + `crossterm`** *(planned, phase 3)* -- TUI rendering and
  terminal backend. Most mature option in the ecosystem, which matters
  most while still learning the immediate-mode rendering model.

</details>

## Contributing

> ✍️ **TODO:** add contribution guidelines once phase 4 gets here, or
> sooner if someone else joins in.
