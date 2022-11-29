import util, { pipe } from "@std/util";
import math from "@std/math";

export abstract class Enum {
  private fnForAll(this: A | B, a) {
    return pipe(this).op(math["+"], a).end;
  }
}

export class A extends Enum {
  public readonly test = `fsldkjf`;
}

export class B extends Enum {
  private readonly val = false;
}
