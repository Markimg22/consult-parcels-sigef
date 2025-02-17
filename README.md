> # Consultar Parcelas SIGEF

> ## Run:

Install dependecies:
```bash
npm install
```

Run with Tauri:
```bash
npm run tauri dev
```

> ## Build a new version:
**🚧 Generate script to automate this process.**

Generate a new version in the frontend project:
```bash
npm version version_number (ex.: 0.1.2)
```

Generate a new version in backend project:
```bash
cargo set-version version_number (ex.: 0.1.2)
```

**🚨 OBS:** Change <kbd>tauri.conf.json</kbd> version to new version

Now you can commit and upload the new version.
