# Todo Dioxus

Eine simple To-Do App gebaut mit:
- **Tauri** - Cross-platform desktop framework
- **Dioxus** - Rust UI framework
- **TailwindCSS** - Utility-first CSS framework

## Features

- ✅ Aufgaben hinzufügen
- ✅ Aufgaben erledigen/abwählen
- ✅ Aufgaben löschen
- ✅ Fortschrittsanzeige
- 📱 Mobile & Desktop Support

## Development

### Voraussetzungen

- Rust & Cargo
- Node.js & npm
- Tauri CLI

### Setup

```bash
# Install Tailwind CSS dependencies
npm install

# Build Tailwind CSS
npm run build

# Run development build
cargo tauri dev
```

### Build für Release

```bash
# Build CSS
npm run build

# Build Tauri app
cargo tauri build
```

## Projektstruktur

```
todo-dioxus/
├── src/              # Dioxus Frontend (Rust)
├── src-tauri/        # Tauri Backend
├── dist/             # Built CSS & assets
├── input.css         # Tailwind input
└── tailwind.config.js # Tailwind Konfiguration
```

## GitHub

https://github.com/Netram2Neo/todo-dioxus
