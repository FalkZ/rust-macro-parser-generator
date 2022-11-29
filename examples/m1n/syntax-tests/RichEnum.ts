enum Types {
  A = "A",
  B = "B",
}

abstract class Rich {
  public abstract readonly type: Types;

  match(this: A | B) {
    if (this.type === Types.A) {
      this.valA;
    }
  }
}

class A extends Rich {
  public readonly type = Types.A as const;

  public valA: string = "hello";
}

class B extends Rich {
  public readonly type = Types.B as const;

  public valB: boolean = true;
}
