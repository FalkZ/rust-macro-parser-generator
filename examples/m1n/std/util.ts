class Pipe<Value1> {
  private value: Value1;
  constructor(value: Value1) {
    this.value = value;
  }

  public op<Value2, Out>(
    op: (a: Value1, b: Value2) => Out,
    value: Value2
  ): Pipe<Out> {
    return new Pipe<Out>(op(this.value, value));
  }

  public get end(): Value1 {
    return this.value;
  }
}

export const pipe = <T>(value: T): Pipe<T> => {
  return new Pipe<T>(value);
};

export const match = <T, R>(value: T, fn: (arg: T) => R): R => {
  return fn(value);
};

export const assign = <T>(value: T, fn: (arg: T) => void): T => {
  fn(value);
  return value;
};

export default { pipe };
