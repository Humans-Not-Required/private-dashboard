const BASE = '/api/v1';

export async function fetchStats() {
  const res = await fetch(`${BASE}/stats`);
  if (!res.ok) throw new Error(`Failed to fetch stats: ${res.status}`);
  return res.json();
}

export async function fetchStatHistory(key, period = '24h') {
  const res = await fetch(`${BASE}/stats/${encodeURIComponent(key)}?period=${period}`);
  if (!res.ok) throw new Error(`Failed to fetch stat history: ${res.status}`);
  return res.json();
}

export async function fetchHealth() {
  const res = await fetch(`${BASE}/health`);
  if (!res.ok) throw new Error(`Failed to fetch health: ${res.status}`);
  return res.json();
}
