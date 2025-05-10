# 🧠 gemx

Terminal-first project, todo, and mindmap management tool inspired by `void-rs`, rebuilt with JSON storage, a shard/tag system, and a full TUI + shortcut layer.

---

## ✨ Features

- 📂 JSON-backed Notes, Todos, Projects
- 🧠 Interactive Mindmap (Keyboard-driven, Shard-aware)
- 🔐 Secure Mode (AES-GCM encryption)
- 🎹 Fully Customizable Keyboard Shortcuts
- 🧩 Plugin-ready architecture

---

## 🧠 Mindmap UI (Shortcuts)

| Action                 | Key        | Action               | Key        |
|------------------------|------------|----------------------|------------|
| Move                  | ↑ ↓ ← →    | Collapse/Expand      | Ctrl+T     |
| Create Child          | Tab        | Create Sibling       | Enter      |
| Raise / Lower         | Ctrl+G / D | Yank / Paste Node    | Ctrl+Y     |
| Drill / Pop Up        | Ctrl+W / Q | Export Snapshot      | Ctrl+E     |
| Search                | Ctrl+U     | Undo Delete          | Ctrl+Z     |
| Help                  | ?          | Save                 | Ctrl+X     |

---

## 📁 Layout

gemx/
├── src/
│ ├── app.rs, main.rs
│ ├── ui/ → dashboard.rs, gemxmap.rs
│ ├── storage/ → note.rs, todo.rs, ingest.rs
│ ├── secure.rs → AES encryption
├── data/ → saved notes/todos/projects
├── keymap.json → customizable keymap
├── settings.json → runtime config


---

## 🚀 Run

```bash
cargo run
To enable secure mode:

cargo run --features secure
📦 License

MIT