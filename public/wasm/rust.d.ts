/* tslint:disable */
/* eslint-disable */
/**
* @param {bigint} seed
* @param {number | undefined} [al]
* @param {number | undefined} [bl]
* @returns {string}
*/
export function generate(seed: bigint, al?: number, bl?: number): string;
/**
* @param {string} input_s
* @param {string} output_s
* @param {number} k
* @returns {SolInfo}
*/
export function get_sol_info(input_s: string, output_s: string, k: number): SolInfo;
/**
* @param {string} _input
* @param {string} _output
* @param {number} t
* @param {number} k
* @returns {VisResult}
*/
export function visualize(_input: string, _output: string, t: number, k: number): VisResult;
/**
* @param {number} seed
* @returns {string}
*/
export function gen(seed: number): string;
/**
* @param {string} _input
* @param {string} _output
* @param {number} turn
* @returns {Ret}
*/
export function vis(_input: string, _output: string, turn: number): Ret;
/**
* @param {string} _input
* @param {string} _output
* @returns {number}
*/
export function get_max_turn(_input: string, _output: string): number;
/**
*/
export class CopySignals {
  free(): void;
/**
*/
  len: number;
/**
*/
  p_a: number;
/**
*/
  p_b: number;
}
/**
*/
export class Ret {
  free(): void;
/**
*/
  err: string;
/**
*/
  score: bigint;
/**
*/
  svg: string;
}
/**
*/
export class SolInfo {
  free(): void;
/**
*/
  error?: string;
/**
*/
  max_turn: number;
/**
*/
  score: number;
}
/**
*/
export class VisResult {
  free(): void;
/**
*/
  a: Uint32Array;
/**
*/
  b: Int32Array;
/**
*/
  comments: (string)[];
/**
*/
  copy_signals?: CopySignals;
/**
*/
  cur_v: number;
/**
*/
  initial_comments: (string)[];
/**
*/
  op?: string;
/**
*/
  score: number;
/**
*/
  svg: string;
/**
*/
  t_sz: number;
/**
*/
  target_idx: number;
/**
*/
  target_v?: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_copysignals_free: (a: number) => void;
  readonly __wbg_get_copysignals_len: (a: number) => number;
  readonly __wbg_set_copysignals_len: (a: number, b: number) => void;
  readonly __wbg_get_copysignals_p_a: (a: number) => number;
  readonly __wbg_set_copysignals_p_a: (a: number, b: number) => void;
  readonly __wbg_get_copysignals_p_b: (a: number) => number;
  readonly __wbg_set_copysignals_p_b: (a: number, b: number) => void;
  readonly generate: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_solinfo_free: (a: number) => void;
  readonly __wbg_get_solinfo_error: (a: number, b: number) => void;
  readonly __wbg_set_solinfo_error: (a: number, b: number, c: number) => void;
  readonly __wbg_get_solinfo_score: (a: number) => number;
  readonly __wbg_set_solinfo_score: (a: number, b: number) => void;
  readonly __wbg_get_solinfo_max_turn: (a: number) => number;
  readonly __wbg_set_solinfo_max_turn: (a: number, b: number) => void;
  readonly get_sol_info: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly __wbg_visresult_free: (a: number) => void;
  readonly __wbg_get_visresult_svg: (a: number, b: number) => void;
  readonly __wbg_set_visresult_svg: (a: number, b: number, c: number) => void;
  readonly __wbg_get_visresult_score: (a: number) => number;
  readonly __wbg_set_visresult_score: (a: number, b: number) => void;
  readonly __wbg_get_visresult_t_sz: (a: number) => number;
  readonly __wbg_set_visresult_t_sz: (a: number, b: number) => void;
  readonly __wbg_get_visresult_target_idx: (a: number) => number;
  readonly __wbg_set_visresult_target_idx: (a: number, b: number) => void;
  readonly __wbg_get_visresult_target_v: (a: number, b: number) => void;
  readonly __wbg_set_visresult_target_v: (a: number, b: number, c: number) => void;
  readonly __wbg_get_visresult_cur_v: (a: number) => number;
  readonly __wbg_set_visresult_cur_v: (a: number, b: number) => void;
  readonly __wbg_get_visresult_op: (a: number, b: number) => void;
  readonly __wbg_set_visresult_op: (a: number, b: number, c: number) => void;
  readonly __wbg_get_visresult_a: (a: number, b: number) => void;
  readonly __wbg_set_visresult_a: (a: number, b: number, c: number) => void;
  readonly __wbg_get_visresult_b: (a: number, b: number) => void;
  readonly __wbg_set_visresult_b: (a: number, b: number, c: number) => void;
  readonly __wbg_get_visresult_copy_signals: (a: number) => number;
  readonly __wbg_set_visresult_copy_signals: (a: number, b: number) => void;
  readonly __wbg_get_visresult_initial_comments: (a: number, b: number) => void;
  readonly __wbg_set_visresult_initial_comments: (a: number, b: number, c: number) => void;
  readonly __wbg_get_visresult_comments: (a: number, b: number) => void;
  readonly __wbg_set_visresult_comments: (a: number, b: number, c: number) => void;
  readonly visualize: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly gen: (a: number, b: number) => void;
  readonly __wbg_ret_free: (a: number) => void;
  readonly __wbg_get_ret_score: (a: number) => number;
  readonly __wbg_set_ret_score: (a: number, b: number) => void;
  readonly __wbg_get_ret_err: (a: number, b: number) => void;
  readonly __wbg_set_ret_err: (a: number, b: number, c: number) => void;
  readonly __wbg_get_ret_svg: (a: number, b: number) => void;
  readonly __wbg_set_ret_svg: (a: number, b: number, c: number) => void;
  readonly vis: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly get_max_turn: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
