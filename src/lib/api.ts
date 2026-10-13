// Typed wrappers over the Rust commands (src-tauri/src/lib.rs). Shapes mirror the
// Rust structs' JSON, so field names stay snake_case.
import { invoke } from "@tauri-apps/api/core";

export class ApiError extends Error {
  /** "auth" (sign in again), "offline" (ZeroTier), "nbs" (NBS refused), "input" (fix the form) */
  constructor(public kind: string, message: string) {
    super(message);
  }
}

async function call<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(cmd, args);
  } catch (e) {
    const err = e as { kind?: string; message?: string };
    throw new ApiError(err?.kind ?? "nbs", err?.message ?? String(e));
  }
}

export type Me = {
  employee_id: number;
  employee_name: string;
  first_name: string;
  display_name: string;
  employee_code: string;
  position: string;
  department: string;
};

export type Profile = {
  employee_name: string;
  first_name: string;
  last_name: string;
  nickname: string;
  employee_code: string;
  position: string;
  department: string;
  division: string;
  team: string;
  branch: string;
  work_location: string;
  pay_group: string;
  employment_type: string;
  date_hired: string;
  employment_tenure: string;
  immediate_superior_employee: string;
  work_email_address: string;
};

export type Tone = "" | "danger" | "attention" | "success" | "muted";
export type Verdict = "" | "Done" | "Waiting" | "Due today" | "Needs action";

export type Filing = {
  status: string;
  deletable: boolean;
  time_in: string;
  time_out: string;
  hours: string;
  reason: string;
  shift: string;
  customer: string;
};

export type Row = {
  date: string;
  label: string;
  today: boolean;
  workday: boolean;
  verdict: Verdict;
  tone: Tone;
  ob_time: string;
  shift: string;
  hours: string;
  note: string;
  reasons: string;
  reasons_tip: string;
  status_tip: string;
  ob: Filing | null;
  cs: Filing | null;
};

export type RailDay = {
  date: string;
  day: number;
  kind: "done" | "wait" | "due" | "act" | "up";
  today: boolean;
  struck: boolean;
  cutoff: boolean;
  tip: string;
};

export type Strip = { tone: Tone; headline: string; detail: string; cutoff: string; progress: string };
export type PeriodView = { label: string; today: string; rows: Row[]; strip: Strip; rail: RailDay[] };
export type ShiftOption = { id: number; name: string; time_in: string; time_out: string };

export type FileRequest = {
  date: string;
  shift_id: number;
  shift_name: string;
  time_in: string;
  time_out: string;
  ob_reason: string;
  cs_reason: string;
};
export type KindOutcome = { state: "created" | "skipped" | "failed"; message: string };
export type FileOutcome = { ob: KindOutcome; cs: KindOutcome };

export const api = {
  serverReachable: () => call<boolean>("server_reachable"),
  restoreSession: () => call<Me | null>("restore_session"),
  login: (username: string, password: string, remember: boolean) => call<Me>("login", { username, password, remember }),
  logout: () => call<void>("logout"),
  profile: () => call<Profile>("profile"),
  loadPeriod: (year: number, month: number, half: number) => call<PeriodView>("load_period", { year, month, half }),
  shiftsFor: (date: string) => call<ShiftOption[]>("shifts_for", { date }),
  fileDay: (request: FileRequest) => call<FileOutcome>("file_day", { request }),
  deleteFilings: (date: string, kinds: string[]) => call<string[]>("delete_filings", { date, kinds }),
};

/** Per-PC conveniences (onboarding seen, last username, reason drafts). Never required. */
export const local = {
  get(key: string, fallback = ""): string {
    try {
      return localStorage.getItem(`nbsbuddy.${key}`) ?? fallback;
    } catch {
      return fallback;
    }
  },
  set(key: string, value: string) {
    try {
      localStorage.setItem(`nbsbuddy.${key}`, value);
    } catch {
      /* private mode / blocked storage: fine */
    }
  },
};
