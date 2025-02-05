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

Generate a new version in the frontend project:
```bash
npm version version_number (ex.: 0.1.2)
```

Generate a new version in backend project:
```bash
cargo set-version version_number (ex.: 0.1.2)
```

Create a new tag in git for the version:
```bash
git tag version_number (v0.1.2)
```
Now you can commit and upload the new version.
