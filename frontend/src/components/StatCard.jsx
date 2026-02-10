import React, { useState } from 'react';
import Sparkline from './Sparkline';

const PERIODS = ['24h', '7d', '30d', '90d'];

function formatNumber(n) {
  if (n >= 1_000_000) return (n / 1_000_000).toFixed(1) + 'M';
  if (n >= 1_000) return (n / 1_000).toFixed(1) + 'K';
  if (Number.isInteger(n)) return n.toLocaleString();
  return n.toFixed(1);
}

function TrendBadge({ trend }) {
  if (!trend || trend.change === null || trend.change === undefined) {
    return <span className="text-xs text-slate-500">—</span>;
  }

  const isUp = trend.change > 0;
  const isDown = trend.change < 0;
  const isFlat = trend.change === 0;

  const color = isUp ? 'text-emerald-400' : isDown ? 'text-red-400' : 'text-slate-400';
  const arrow = isUp ? '↑' : isDown ? '↓' : '→';
  const pct = trend.pct !== null && trend.pct !== undefined
    ? `${Math.abs(trend.pct).toFixed(1)}%`
    : '';

  return (
    <span className={`text-xs font-medium ${color}`}>
      {arrow} {formatNumber(Math.abs(trend.change))} {pct && `(${pct})`}
    </span>
  );
}

export default function StatCard({ stat }) {
  const [period, setPeriod] = useState('24h');
  const trend = stat.trends?.[period];
  const sparkColor = trend?.change > 0 ? '#34d399' : trend?.change < 0 ? '#f87171' : '#60a5fa';

  return (
    <div className="bg-slate-900 border border-slate-800 rounded-2xl p-5 flex flex-col gap-3 hover:border-slate-700 transition-colors">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium text-slate-400 uppercase tracking-wide">
          {stat.label}
        </h3>
      </div>

      {/* Big Number */}
      <div className="text-4xl font-bold text-white tabular-nums">
        {formatNumber(stat.current)}
      </div>

      {/* Trend */}
      <div className="flex items-center gap-2">
        <TrendBadge trend={trend} />
      </div>

      {/* Sparkline */}
      <div className="mt-1">
        <Sparkline data={stat.sparkline_24h} width={200} height={48} color={sparkColor} />
      </div>

      {/* Period Selector */}
      <div className="flex gap-1 mt-auto">
        {PERIODS.map(p => (
          <button
            key={p}
            onClick={() => setPeriod(p)}
            className={`text-xs px-2 py-1 rounded-md transition-colors ${
              p === period
                ? 'bg-slate-700 text-white'
                : 'text-slate-500 hover:text-slate-300 hover:bg-slate-800'
            }`}
          >
            {p}
          </button>
        ))}
      </div>

      {/* Last updated */}
      <div className="text-[10px] text-slate-600">
        {stat.last_updated ? new Date(stat.last_updated).toLocaleTimeString() : '—'}
      </div>
    </div>
  );
}
