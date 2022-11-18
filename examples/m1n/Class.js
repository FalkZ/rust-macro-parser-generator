"use strict";
import math from "./math";
import { op } from "./std";
export class Class {
  fn(a2, b2) {
    return op(a2, math["+"], b2);
  }
  one(b2) {
    return op(op(a, math["+"], b2), math["+"], c);
  }
  zero() {
    return (() => {
      throw new Error("hello 2");
    })();
  }
  multiline() {
    return console.log(
      `test space`,
      `multi

    line`
    );
  }
  nesting() {
    return one(
      fn(
        op(op(a, math["+"], b), math["-"], c),
        123.2
      )
    );
  }
}
//# sourceMappingURL=Class.js.map
