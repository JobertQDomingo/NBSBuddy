// Semi-monthly payroll periods: half 1 = 1st-15th, half 2 = 16th-end of month.

export type Period = { year: number; month: number; half: number };

export const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

export const lastDay = (year: number, month: number) => new Date(year, month, 0).getDate();

export function currentPeriod(d = new Date()): Period {
  return { year: d.getFullYear(), month: d.getMonth() + 1, half: d.getDate() <= 15 ? 1 : 2 };
}

/** Move `delta` half-months; crosses month and year ends. */
export function stepPeriod(p: Period, delta: number): Period {
  const i = p.year * 24 + (p.month - 1) * 2 + (p.half - 1) + delta;
  return { year: Math.floor(i / 24), month: Math.floor((i % 24) / 2) + 1, half: (i % 2) + 1 };
}

export const samePeriod = (a: Period, b: Period) => a.year === b.year && a.month === b.month && a.half === b.half;

export const halfRange = (p: Period) => (p.half === 1 ? "1–15" : `16–${lastDay(p.year, p.month)}`);

/** "Jul 1–15, 2026" */
export const periodLabel = (p: Period) => `${MONTHS[p.month - 1]} ${halfRange(p)}, ${p.year}`;

/** Local "YYYY-MM-DD" (not UTC: toISOString would shift the day near midnight). */
export function isoDate(d = new Date()): string {
  const pad = (n: number) => String(n).padStart(2, "0");
  return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
}

export const parseIso = (iso: string) => new Date(`${iso}T00:00:00`);

/** "Tue Jul 7" */
export const dayLabel = (iso: string) =>
  parseIso(iso).toLocaleDateString("en-US", { weekday: "short", month: "short", day: "numeric" }).replace(",", "");
