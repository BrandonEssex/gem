🧠 gemx

gemx is a fast, tree-first, keyboard-driven planning tool built around a powerful mindmap core. Inspired by void-rs, gemx emphasizes clarity, modularity, and speed—built for engineers, writers, and thinkers who organize ideas as living trees.

✨ Features

🌳 Mindmap-Centric Core – All content is structured as a tree. The mindmap is always active.
🎛 Modular Architecture – Plugins, panels, and features can be fully disabled for performance.
⌨️ Keyboard-Only Control – Inspired by terminal editors, everything is keybind-driven.
📦 Workspaces & Snapshots – Save, sync, rewind, and version your environments.
🧩 Plugin Runtime – Load enhancements dynamically with safe sandboxing.
🔁 Command Mode – Slash (/) and colon (:) command support with autocomplete.
📅 Calendar, Kanban, Timeline – Optional views for different workstyles.
📊 Metrics & Journaling – Visualize, track, and document your thinking.
🧰 Installation

git clone https://github.com/yourname/gemx.git
cd gemx
cargo build --release
./install.sh
Then run:

gemx
For minimal mode:
gemx --headless :mindmap only
🚀 Quick Start

Keybinding	Action
Ctrl+N	New free-floating node
Enter	Create sibling node
Tab	Create child node
Esc	Unselect node / exit mode
:	Open command mode
/	Open slash command editor
Ctrl+P	Open command palette
Ctrl+Shift+R	Reload layout/theme
?	Shortcut help overlay
🗂 Workspace Structure

~/.config/gemx/
├── settings.json       # UI + feature toggles
├── keymap.json         # Customizable shortcuts
├── themes/             # Theme files (JSON)
├── workspaces/
│   └── project_alpha.json
├── snapshots/
├── plugins/
├── vaults/
└── metrics/
🔌 Plugin System

Plugin Manifest Example
[plugin]
name = "Pomodoro"
entry = "libpomodoro.so"
version = "0.2.1"
capabilities = ["sidebar", "on_node_focus"]
You can enable, disable, and pin plugins via:

:plugin install ./plugins/pomodoro/
:plugin config Pomodoro
🔒 Optional Features

Module	Purpose	Toggle in settings.json
Plugins	Dynamic extension runtime	features.plugins
Vaults	Encrypted workspace archiving	features.vault
Metrics	Node usage tracking + heatmap views	features.metrics
AI Assist	Local or remote smart commands	features.ai
Command Bar	Autocomplete shell for navigation	features.command_bar
📦 Export / Sync / Vault

:workspace export core
:workspace export full
:vault build roadmap
:snapshot rewind <id>
Use --headless for scripts:

gemx --headless ":remind list"
gemx --headless ":export svg"
📜 Philosophy

Everything is a tree 🌲
Features are enhancements, never assumptions
Performance-first, visual-second
Inspired by void-rs, improved for extensibility
📄 License

MIT License © 2025 [Your Name or Org]
See LICENSE for full terms.

