type Corner = "tl" | "tr" | "bl" | "br";

interface Props {
  corner: Corner;
  size?: number;
}

const ROTATIONS: Record<Corner, number> = {
  tl: 0,
  tr: 90,
  br: 180,
  bl: 270,
};

const POSITIONS: Record<Corner, { top?: number; left?: number; right?: number; bottom?: number }> = {
  tl: { top: 0, left: 0 },
  tr: { top: 0, right: 0 },
  bl: { bottom: 0, left: 0 },
  br: { bottom: 0, right: 0 },
};

export function CornerWeb({ corner, size = 260 }: Props) {
  const rotation = ROTATIONS[corner];
  const pos = POSITIONS[corner];

  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      className="fixed pointer-events-none"
      style={{
        ...pos,
        opacity: 0.15,
        transform: `rotate(${rotation}deg)`,
      }}
    >
      {/* radial threads */}
      {[0, 15, 30, 45, 60, 75, 90].map((angle) => (
        <line
          key={`r-${angle}`}
          x1={0}
          y1={0}
          x2={100}
          y2={0}
          transform={`rotate(${angle})`}
          stroke="#c77dff"
          strokeWidth={0.5}
        />
      ))}
      {/* arcs */}
      {[25, 45, 65, 85].map((r) => (
        <path
          key={`a-${r}`}
          d={`M ${r} 0 A ${r} ${r} 0 0 1 0 ${r}`}
          fill="none"
          stroke="#c77dff"
          strokeWidth={0.5}
        />
      ))}
    </svg>
  );
}
