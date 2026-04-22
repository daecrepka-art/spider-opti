import { useEffect, useRef } from "react";

export interface LogEntry {
  level: "info" | "warn" | "error";
  msg: string;
  ts: number;
}

interface Props {
  logs: LogEntry[];
}

const LEVEL_COLOR: Record<LogEntry["level"], string> = {
  info: "#c77dff",
  warn: "#ffb703",
  error: "#ff6b6b",
};

export function SpiderConsole({ logs }: Props) {
  const ref = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    if (ref.current) {
      ref.current.scrollTop = ref.current.scrollHeight;
    }
  }, [logs]);

  return (
    <div
      className="fixed bottom-0 left-0 right-0 h-48 border-t backdrop-blur-md"
      style={{
        background: "rgba(13, 6, 33, 0.72)",
        borderColor: "rgba(199,125,255,0.2)",
      }}
    >
      <div className="px-4 py-2 text-xs uppercase tracking-wider opacity-60">
        Консоль паука
      </div>
      <div
        ref={ref}
        className="spider-console px-4 pb-3 h-[calc(100%-2rem)] overflow-y-auto font-mono text-xs leading-relaxed"
      >
        {logs.length === 0 ? (
          <div className="opacity-40">// тишина... паук спит</div>
        ) : (
          logs.map((log, i) => (
            <div key={i} className="flex gap-3">
              <span className="opacity-50">{formatTs(log.ts)}</span>
              <span style={{ color: LEVEL_COLOR[log.level] }}>
                [{log.level}]
              </span>
              <span className="flex-1">{log.msg}</span>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

function formatTs(ts: number): string {
  const d = new Date(ts * 1000);
  return d.toLocaleTimeString();
}
