import math from "./math";

type Alternating<T extends readonly any[], A, B> = T extends readonly []
  ? T
  : T extends readonly [A]
  ? T
  : T extends readonly [A, B, ...infer T2]
  ? T2 extends Alternating<T2, A, B>
    ? T
    : never
  : never;

export const op = <T extends readonly any[]>(
  value: boolean,
  ...rest: Alternating<T, number, boolean>
) => {};

op(true, true, true);

math["+"];
