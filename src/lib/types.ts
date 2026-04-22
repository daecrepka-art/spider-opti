// Mirrors src-tauri structs; keep in sync with the Rust side.

export type JunkCategory =
  | "temp"
  | "prefetch"
  | "thumb_cache"
  | "inet_cache"
  | "windows_temp";

export interface JunkItem {
  id: string;
  path: string;
  size: number;
  category: JunkCategory;
}

export interface CleanTarget {
  id: string;
  path: string;
}

export interface FailedItem {
  path: string;
  reason: string;
}

export interface CleanReport {
  freed_bytes: number;
  failed: FailedItem[];
}

export interface DiskInfo {
  name: string;
  mount_point: string;
  total_bytes: number;
  available_bytes: number;
}

export interface SystemInfo {
  cpu_usage: number;
  ram_total_bytes: number;
  ram_used_bytes: number;
  disks: DiskInfo[];
  os_name: string;
  os_version: string;
}

export interface ServiceInfo {
  name: string;
  display_name: string;
  start_type: string;
  status: string;
}

export interface ServiceState {
  name: string;
  start_type: string;
  status: string;
}

export interface RegistryReport {
  removed: string[];
  skipped: string[];
}
