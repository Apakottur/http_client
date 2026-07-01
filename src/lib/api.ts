import { invoke } from "@tauri-apps/api/core";
import type { Collection, Request, HttpResponse } from "./types";

export const loadCollection = () => invoke<Collection>("load_collection");
export const saveCollection = (collection: Collection) => invoke<void>("save_collection", { collection });
export const sendRequest = (request: Request) => invoke<HttpResponse>("send_request", { request });
export const getConfigPath = () => invoke<string>("get_config_path");
export const setConfigPath = (path: string) => invoke<Collection>("set_config_path", { path });
