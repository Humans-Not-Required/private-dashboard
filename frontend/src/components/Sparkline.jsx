import React, { useState, useRef, useCallback } from 'react';

export default function Sparkline({ data, color = '#60a5fa', timestamps = null }) {
  const [hover, setHover] = useState(null);
  const svgRef = useRef(null);

  if (!data || data.length < 2) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <span className="text-[10px] text-slate-700 italic">awaiting data…</span>
      </div>
    );
  }

  const min = Math.min(...data);
  const max = Math.max(...data);
  const range = max - min || 1;
  const padding = 2;

  const vw = 300;
  const vh = 80;

  const getCoords = (val, i) => {
    const x = padding + (i / (data.length - 1)) * (vw - padding * 2);
    const y = vh - padding - ((val - min) / range) * (vh - padding * 2);
    return { x, y };
  };

  const points = data.map((val, i) => {
    const { x, y } = getCoords(val, i);
    return `${x},${y}`;
  }).join(' ');

  const firstX = padding;
  const lastX = padding + ((data.length - 1) / (data.length - 1)) * (vw - padding * 2);
  const areaPoints = `${firstX},${vh} ${points} ${lastX},${vh}`;

  const gradId = `grad-${color.replace('#', '')}`;

  const handleMouseMove = useCallback((e) => {
    const svg = svgRef.current;
    if (!svg) return;
    const rect = svg.getBoundingClientRect();
    const mouseX = ((e.clientX - rect.left) / rect.width) * vw;
    // Find nearest data point
    let nearest = 0;
    let minDist = Infinity;
    for (let i = 0; i < data.length; i++) {
      const { x } = getCoords(data[i], i);
      const dist = Math.abs(x - mouseX);
      if (dist < minDist) {
        minDist = dist;
        nearest = i;
      }
    }
    const { x, y } = getCoords(data[nearest], nearest);
    setHover({ idx: nearest, x, y, value: data[nearest] });
  }, [data]);

  const handleMouseLeave = useCallback(() => setHover(null), []);

  // Format value for tooltip
  const fmtVal = (v) => {
    if (Number.isInteger(v)) return v.toLocaleString();
    return v.toFixed(1);
  };

  // Format timestamp for tooltip
  const fmtTime = (idx) => {
    if (!timestamps || !timestamps[idx]) return '';
    try {
      const d = new Date(timestamps[idx]);
      return d.toLocaleDateString([], { month: 'short', day: 'numeric' }) + ' ' +
             d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    } catch { return ''; }
  };

  // Last point dot
  const lastVal = data[data.length - 1];
  const lastCoords = getCoords(lastVal, data.length - 1);

  return (
    <svg
      ref={svgRef}
      viewBox={`0 0 ${vw} ${vh}`}
      preserveAspectRatio="none"
      className="w-full h-full cursor-crosshair"
      onMouseMove={handleMouseMove}
      onMouseLeave={handleMouseLeave}
    >
      <defs>
        <linearGradient id={gradId} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor={color} stopOpacity="0.25" />
          <stop offset="100%" stopColor={color} stopOpacity="0.02" />
        </linearGradient>
      </defs>
      <polygon points={areaPoints} fill={`url(#${gradId})`} />
      <polyline
        points={points}
        fill="none"
        stroke={color}
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        vectorEffect="non-scaling-stroke"
      />
      {/* End dot */}
      <circle cx={lastCoords.x} cy={lastCoords.y} r="3" fill={color} opacity="0.8" />

      {/* Hover crosshair + tooltip */}
      {hover && (
        <>
          <line
            x1={hover.x} y1={0} x2={hover.x} y2={vh}
            stroke="#475569" strokeWidth="1" strokeDasharray="3,3"
            vectorEffect="non-scaling-stroke"
          />
          <circle cx={hover.x} cy={hover.y} r="4" fill={color} stroke="#fff" strokeWidth="1.5"
            vectorEffect="non-scaling-stroke" />
          {/* Tooltip background */}
          <rect
            x={hover.x < vw / 2 ? hover.x + 8 : hover.x - 88}
            y={Math.max(2, hover.y - 28)}
            width="80" height={timestamps ? "30" : "20"}
            rx="4" fill="#1e293b" fillOpacity="0.95" stroke="#334155" strokeWidth="0.5"
          />
          {/* Tooltip value */}
          <text
            x={hover.x < vw / 2 ? hover.x + 14 : hover.x - 82}
            y={Math.max(2, hover.y - 28) + 13}
            fill="#f1f5f9" fontSize="10" fontWeight="600" fontFamily="system-ui"
          >
            {fmtVal(hover.value)}
          </text>
          {/* Tooltip timestamp */}
          {timestamps && (
            <text
              x={hover.x < vw / 2 ? hover.x + 14 : hover.x - 82}
              y={Math.max(2, hover.y - 28) + 25}
              fill="#94a3b8" fontSize="7" fontFamily="system-ui"
            >
              {fmtTime(hover.idx)}
            </text>
          )}
        </>
      )}
    </svg>
  );
}
