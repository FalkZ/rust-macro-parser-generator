import math from "./math";
import { op } from "./std";

export class Class {
  private fn(a, b) {
    return op(a, math["+"], b);
  }

  private one(b) {
    return op(op(a, math["+"], b), math["+"], c);
  }

  public zero() {
    return (() => {
      throw new Error("hello 2");
    })();
  }

  private multiline() {
    return console.log(
      `test space`,
      `multi

    line`
    );
  }

  private nesting() {
    return one(
      fn(
        op(op(a, math["+"], b), math["-"], c),
        123.2
      )
    );
  }
}
//# sourceMappingURL=Class.m1n.map
