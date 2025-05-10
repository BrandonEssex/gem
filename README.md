# 🧠 gemx v0.0.1

A modular terminal-based productivity tool, forked from `void/void-rs`, rebuilt with JSON storage, tagging, and a dashboard-centric TUI interface.

## ✅ Features in v0.0.1

- JSON storage for notes, todos, and projects
- `shard` (single-category) and `tags` (multi) system
- Customizable shortcut keymap (via `keymap.json`)
- `incoming/` folder to ingest `.md` files
- TUI dashboard with live data and shortcuts
- Modular codebase: ready for plugin expansion

## 🚀 Usage

```bash
cargo run
🗂 Directory Layout

src/ – Source code (modularized)
data/ – Stored items (generated on run)
incoming/ – Drop .md files for ingestion
keymap.json – Shortcut mappings
settings.json – Optional global config
🔧 Default Shortcuts

Action	Shortcut
Quit	ctrl+q
Toggle Help	ctrl+h
Start Pomodoro	ctrl+p
Stop Pomodoro	ctrl+s
Toggle Timer	ctrl+t
Open Dashboard	ctrl+d
📦 Future Enhancements

Mindmap view (gemxmap)
Encryption & secure mode
Plugin/extension runtime
Time tracking reports
MIT License. Built with ❤️ for engineers & thinkers.


---

✅ **gemx v0.0.1 complete. All modules finalized and delivered.**
Would you like to begin `v0.0.2` planning or start building toward plugin support and secure mode?