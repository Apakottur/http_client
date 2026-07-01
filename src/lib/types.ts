export interface KeyValue { name: string; value: string; enabled: boolean; }
export interface Auth { type: "none" | "basic" | "bearer"; username: string; password: string; token: string; }
export interface MultipartField { name: string; value: string; kind: "text" | "file"; enabled: boolean; }
export interface Body {
  type: "none" | "json" | "form" | "multipart";
  json: string; form: KeyValue[]; multipart: MultipartField[];
}
export interface Request {
  id: string; name: string; method: string; url: string;
  queryParams: KeyValue[]; headers: KeyValue[]; auth: Auth; body: Body; sortKey: number;
}
export type Theme = "system" | "light" | "dark";
export interface Settings { theme: Theme; }
export interface Collection { version: number; settings: Settings; requests: Request[]; }
export interface HttpResponse {
  status: number; statusText: string; timeMs: number; size: number;
  headers: KeyValue[]; body: string;
}

export function newRequest(): Request {
  return {
    id: crypto.randomUUID(),
    name: "New Request", method: "GET", url: "", queryParams: [], headers: [],
    auth: { type: "none", username: "", password: "", token: "" },
    body: { type: "none", json: "", form: [], multipart: [] },
    sortKey: Date.now(),
  };
}
