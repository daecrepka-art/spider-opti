import { invoke } from "@tauri-apps/api/core";
import type {
  CleanReport,
  CleanTarget,
  JunkItem,
  RegistryReport,
  ServiceInfo,
  ServiceState,
  SystemInfo,
} from "./types";

// Thin typed wrappers around Tauri `invoke` so the component layer never sees
// raw string command names.
export const api = {
  getSystemInfo: () => invoke<SystemInfo>("get_system_info"),
  scanJunkFiles: () => invoke<JunkItem[]>("scan_junk_files"),
  cleanSelected: (items: CleanTarget[]) =>
    invoke<CleanReport>("clean_selected", { items }),
  toggleService: (name: string, enable: boolean) =>
    invoke<ServiceState>("toggle_service", { name, enable }),
  cleanRegistry: () => invoke<RegistryReport>("clean_registry"),
  listDestructiveServices: () =>
    invoke<ServiceInfo[]>("list_destructive_services"),
  createRestorePoint: (description: string) =>
    invoke<void>("create_restore_point", { description }),
};
