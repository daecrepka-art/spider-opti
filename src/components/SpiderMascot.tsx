import { motion, AnimatePresence, Variants } from "framer-motion";
import { memo } from "react";

export type SpiderState = "idle" | "working" | "done";

interface Props {
  state: SpiderState;
  size?: number;
}

const bodyVariants: Variants = {
  idle: {
    y: [0, -8, 0],
    transition: { duration: 3, repeat: Infinity, ease: "easeInOut" },
  },
  working: {
    y: [0, 30, 0],
    transition: { duration: 0.8, repeat: Infinity, ease: "easeInOut" },
  },
  done: {
    scale: [1, 1.4, 1],
    transition: { duration: 0.9, ease: "easeOut" },
  },
};

const legContainer: Variants = {
  idle: { transition: { staggerChildren: 0.1 } },
  working: { transition: { staggerChildren: 0.05 } },
  done: { transition: { staggerChildren: 0.08 } },
};

const legVariants: Variants = {
  idle: {
    rotate: [-3, 3, -3],
    transition: { duration: 3, repeat: Infinity, ease: "easeInOut" },
  },
  working: {
    rotate: [-20, 20, -20],
    transition: { duration: 0.4, repeat: Infinity, ease: "easeInOut" },
  },
  done: {
    rotate: 0,
    transition: { duration: 0.3 },
  },
};

const RAYS = 8;

function SpiderMascotInner({ state, size = 180 }: Props) {
  return (
    <div
      className="relative flex items-center justify-center"
      style={{ width: size, height: size }}
    >
      {/* Web flash on done */}
      <AnimatePresence>
        {state === "done" && (
          <motion.svg
            key="web-flash"
            width={size * 2}
            height={size * 2}
            viewBox="-100 -100 200 200"
            className="absolute pointer-events-none"
            initial={{ opacity: 0, scale: 0.6 }}
            animate={{ opacity: [0, 0.9, 0], scale: [0.6, 1.3, 1.5] }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.9, ease: "easeOut" }}
          >
            {Array.from({ length: RAYS }).map((_, i) => {
              const angle = (i * 360) / RAYS;
              return (
                <line
                  key={i}
                  x1={0}
                  y1={0}
                  x2={0}
                  y2={-90}
                  stroke="#c77dff"
                  strokeWidth={1.2}
                  strokeLinecap="round"
                  transform={`rotate(${angle})`}
                  opacity={0.85}
                />
              );
            })}
          </motion.svg>
        )}
      </AnimatePresence>

      {/* Spider body + legs */}
      <motion.div
        variants={legContainer}
        animate={state}
        className="relative"
        style={{ width: size, height: size }}
      >
        <motion.svg
          variants={bodyVariants}
          animate={state}
          viewBox="-50 -50 100 100"
          width={size}
          height={size}
        >
          {[-3, -1.5, 1.5, 3].map((yOffset, idx) => (
            <motion.g key={`L${idx}`} variants={legVariants}>
              <path
                d={`M -10 ${yOffset * 4} Q -30 ${yOffset * 4 - 10} -40 ${yOffset * 4 + 20}`}
                stroke="#9d4edd"
                strokeWidth={2.5}
                fill="none"
                strokeLinecap="round"
              />
            </motion.g>
          ))}
          {[-3, -1.5, 1.5, 3].map((yOffset, idx) => (
            <motion.g key={`R${idx}`} variants={legVariants}>
              <path
                d={`M 10 ${yOffset * 4} Q 30 ${yOffset * 4 - 10} 40 ${yOffset * 4 + 20}`}
                stroke="#9d4edd"
                strokeWidth={2.5}
                fill="none"
                strokeLinecap="round"
              />
            </motion.g>
          ))}

          <ellipse cx={0} cy={2} rx={14} ry={11} fill="#c77dff" />
          <circle cx={0} cy={-10} r={9} fill="#9d4edd" />
          <circle cx={-3} cy={-11} r={1.8} fill="#fff" />
          <circle cx={3} cy={-11} r={1.8} fill="#fff" />
          <circle cx={-3} cy={-11} r={0.9} fill="#1a102e" />
          <circle cx={3} cy={-11} r={0.9} fill="#1a102e" />
        </motion.svg>
      </motion.div>
    </div>
  );
}

export const SpiderMascot = memo(SpiderMascotInner);
