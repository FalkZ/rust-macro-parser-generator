import util, { pipe } from "@std/util";
import math from "@std/math";

export abstract class Enum {
  protected fnForAll(this: A | B, a) {
    return math["+"](this, a);
  }
}

export class A extends Enum {
  public readonly test = `fsldkjf`;
}

export class B extends Enum {
  private readonly val = false;
}
