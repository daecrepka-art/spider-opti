import { useEffect, useState } from "react";
import { AnimatePresence, motion } from "framer-motion";
import { listen } from "@tauri-apps/api/event";

import { SpiderMascot, SpiderState } from "./components/SpiderMascot";
import { CornerWeb } from "./components/CornerWeb";
import { CategoryCard } from "./components/CategoryCard";
import { SpiderConsole, LogEntry } from "./components/SpiderConsole";
import { api } from "./lib/tauriApi";

interface Selection {
  junk: boolean;
  registry: boolean;
  services: boolean;
}

export default function App() {
  const [spider, setSpider] = useState<SpiderState>("idle");
  const [status, setStatus] = useState("Паук готов к работе");
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [selected, setSelected] = useState<Selection>({
    junk: true,
    registry: false,
    services: false,
  });
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    const un = listen<LogEntry>("spider://log", (e) => {
      setLogs((prev) => [...prev.slice(-299), e.payload]);
    });
    return () => {
      un.then((f) => f()).catch(() => {});
    };
  }, []);

  async function runCleanup() {
    if (busy) return;
    setBusy(true);
    try {
      setSpider("working");
      setStatus("Паук плетёт паутину по вашей системе...");

      if (selected.junk) {
        const junk = await api.scanJunkFiles();
        if (junk.length > 0) {
          setStatus("Сжигаю мусор...");
          await api.cleanSelected(
            junk.map((j) => ({ id: j.id, path: j.path })),
          );
        }
      }

      if (selected.registry) {
        setStatus("Прочищаю нити реестра...");
        await api.cleanRegistry();
      }

      setSpider("done");
      setStatus("Готово. Система стала шустрее.");
      setTimeout(() => setSpider("idle"), 1200);
    } catch (e) {
      setSpider("idle");
      setStatus(`Ошибка: ${String(e)}`);
    } finally {
      setBusy(false);
    }
  }

  return (
    <div
      className="min-h-screen w-full text-white font-sans relative overflow-hidden"
      style={{
        background:
          "radial-gradient(circle at 50% 30%, #1a102e 0%, #0d0621 80%)",
      }}
    >
      <CornerWeb corner="tl" />
      <CornerWeb corner="tr" />
      <CornerWeb corner="bl" />
      <CornerWeb corner="br" />

      <main className="relative z-10 flex flex-col items-center pt-16 pb-64 px-6 gap-10">
        <SpiderMascot state={spider} size={220} />

        <AnimatePresence mode="wait">
          <motion.p
            key={status}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            transition={{ duration: 0.35 }}
            className="text-lg tracking-wide"
            style={{ color: "var(--accent-bright)" }}
          >
            {status}
          </motion.p>
        </AnimatePresence>

        <section className="grid grid-cols-1 md:grid-cols-3 gap-5 w-full max-w-4xl">
          <CategoryCard
            title="Мусорные файлы"
            description="Temp, Prefetch, Thumbnail-кеш"
            checked={selected.junk}
            onToggle={() => setSelected((s) => ({ ...s, junk: !s.junk }))}
          />
          <CategoryCard
            title="Реестр"
            description="Только безопасный WHITELIST"
            checked={selected.registry}
            onToggle={() =>
              setSelected((s) => ({ ...s, registry: !s.registry }))
            }
          />
          <CategoryCard
            title="Фоновые службы"
            description="Телеметрия, Xbox, SuperFetch"
            checked={selected.services}
            onToggle={() =>
              setSelected((s) => ({ ...s, services: !s.services }))
            }
          />
        </section>

        <button
          onClick={runCleanup}
          disabled={busy}
          className="px-8 py-3 rounded-full font-semibold tracking-wide transition-all disabled:opacity-50"
          style={{
            background:
              "linear-gradient(135deg, var(--accent), var(--accent-bright))",
            boxShadow: "0 0 24px rgba(199,125,255,0.35)",
          }}
        >
          {busy ? "Паук работает..." : "Запустить паука"}
        </button>
      </main>

      <SpiderConsole logs={logs} />
    </div>
  );
}
