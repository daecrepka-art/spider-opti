# Spider-Opti

Windows optimizer with a spider mascot. Tauri 2 + React + Framer Motion.

## Prerequisites

- Windows 10 / 11 (the app is Windows-only by design)
- Node 20+
- Rust stable (MSVC toolchain recommended)
- Admin rights (the bundled manifest requests them)

## Development

```bash
npm install
cargo tauri dev
```

## Production build

```bash
cargo tauri build
```

Produces an `.msi` + `.exe` in `src-tauri/target/release/bundle/`.

## Architecture

- `src-tauri/src/lib.rs` — registers every Tauri command.
- `src-tauri/src/scanner.rs` — parallel junk-file scan via `rayon`.
- `src-tauri/src/cleaner.rs` — deletes files only inside an allow-list of roots.
- `src-tauri/src/services.rs` — start / stop / disable services via `windows-service`.
- `src-tauri/src/registry.rs` — touches only whitelisted HKCU branches.
- `src-tauri/src/restore_point.rs` — creates a System Restore point via `Checkpoint-Computer` before any destructive action.
- `src/` — React UI: animated spider mascot, glassmorphism category cards, live "spider console" log.

## Safety

- Every destructive action (file cleanup, registry cleanup) creates a restore point first.
- Registry writes are restricted to a static whitelist — HKLM and `Services` are never touched.
- File deletion is restricted to `%TEMP%`, `%LOCALAPPDATA%\Microsoft\Windows\INetCache`, `%SystemRoot%\Temp` and `%SystemRoot%\Prefetch`.
- `ERROR_ACCESS_DENIED` is surfaced to the UI with a dedicated error variant.
