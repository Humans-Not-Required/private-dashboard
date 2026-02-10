import React, { useState, useEffect } from 'react';
import { fetchStats, fetchHealth } from './api';
import StatCard from './components/StatCard';

const REFRESH_INTERVAL = 60_000; // 60 seconds

export default function App() {
  const [stats, setStats] = useState([]);
  const [health, setHealth] = useState(null);
  const [error, setError] = useState(null);
  const [lastRefresh, setLastRefresh] = useState(null);

  const loadData = async () => {
    try {
      const [statsData, healthData] = await Promise.all([
        fetchStats(),
        fetchHealth(),
      ]);
      setStats(statsData.stats || []);
      setHealth(healthData);
      setError(null);
      setLastRefresh(new Date());
    } catch (err) {
      setError(err.message);
    }
  };

  useEffect(() => {
    loadData();
    const interval = setInterval(loadData, REFRESH_INTERVAL);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="min-h-screen bg-slate-950 text-white p-6">
      {/* Header */}
      <header className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-2xl font-bold">📊 HNR Dashboard</h1>
          <p className="text-sm text-slate-500 mt-1">
            Humans Not Required — Operational Metrics
          </p>
        </div>
        <div className="text-right text-xs text-slate-500">
          {health && (
            <div>{health.keys_count} metrics · {health.stats_count} data points</div>
          )}
          {lastRefresh && (
            <div>Updated {lastRefresh.toLocaleTimeString()}</div>
          )}
          {error && (
            <div className="text-red-400 mt-1">⚠ {error}</div>
          )}
        </div>
      </header>

      {/* Stats Grid */}
      {stats.length === 0 ? (
        <div className="flex items-center justify-center h-[60vh]">
          <div className="text-center">
            <div className="text-6xl mb-4">📡</div>
            <h2 className="text-xl text-slate-400 mb-2">No Data Yet</h2>
            <p className="text-sm text-slate-600 max-w-md">
              Stats will appear here once the collector cron starts posting data.
              Use <code className="bg-slate-800 px-1.5 py-0.5 rounded text-xs">POST /api/v1/stats</code> to submit metrics.
            </p>
          </div>
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
          {stats.map(stat => (
            <StatCard key={stat.key} stat={stat} />
          ))}
        </div>
      )}
    </div>
  );
}
