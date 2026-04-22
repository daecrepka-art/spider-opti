import { motion } from "framer-motion";
import { WebCheckbox } from "./WebCheckbox";

interface Props {
  title: string;
  description: string;
  checked: boolean;
  onToggle: () => void;
}

export function CategoryCard({ title, description, checked, onToggle }: Props) {
  return (
    <motion.button
      type="button"
      whileHover={{ y: -4 }}
      whileTap={{ scale: 0.98 }}
      onClick={onToggle}
      className="text-left w-full cursor-pointer rounded-2xl p-5 border transition-colors focus:outline-none focus:ring-2"
      style={{
        background: "var(--glass)",
        backdropFilter: "blur(12px)",
        WebkitBackdropFilter: "blur(12px)",
        borderColor: checked
          ? "rgba(199,125,255,0.6)"
          : "rgba(199,125,255,0.25)",
      }}
    >
      <div className="flex items-start justify-between gap-3">
        <div>
          <h3 className="text-base font-semibold">{title}</h3>
          <p className="text-sm opacity-70 mt-1">{description}</p>
        </div>
        <WebCheckbox checked={checked} />
      </div>
    </motion.button>
  );
}
