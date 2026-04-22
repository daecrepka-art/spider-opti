import { motion } from "framer-motion";

interface Props {
  checked: boolean;
}

/**
 * Decorative checkbox styled as a spider-web junction.
 * The click handler lives on the parent card so the whole card is clickable.
 */
export function WebCheckbox({ checked }: Props) {
  return (
    <div className="relative w-6 h-6 shrink-0">
      <svg viewBox="-12 -12 24 24" className="w-full h-full">
        {/* radial threads */}
        {[0, 45, 90, 135].map((angle) => (
          <line
            key={angle}
            x1={-10}
            y1={0}
            x2={10}
            y2={0}
            transform={`rotate(${angle})`}
            stroke="rgba(199,125,255,0.35)"
            strokeWidth={0.7}
          />
        ))}
        {/* outer ring */}
        <circle
          cx={0}
          cy={0}
          r={9}
          fill="none"
          stroke="rgba(199,125,255,0.55)"
          strokeWidth={1.2}
        />
      </svg>
      <motion.div
        className="absolute inset-1 rounded-full"
        initial={false}
        animate={{
          scale: checked ? 1 : 0,
          opacity: checked ? 1 : 0,
        }}
        transition={{ duration: 0.2, ease: "easeOut" }}
        style={{
          background:
            "radial-gradient(circle, #c77dff 0%, #9d4edd 70%, transparent 100%)",
          boxShadow: "0 0 12px rgba(199,125,255,0.7)",
        }}
      />
    </div>
  );
}
