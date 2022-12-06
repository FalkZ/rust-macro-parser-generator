# Body

```python

PI = a + b + 0.012;

pub mut Pi = 123;

fn(a, b) = a + b;

pub `+`(a, b) = a + b;


one(b) = console.log(b);

pub zero() = a + b;

```

# Imports

```python
version: v1;

import:
    `std/math`,
    `std/math`{`+`};

derive: equals, to_json;


```

# Literals

```python

obj = {
    val = 1,
    val2 = "dfsfd"
};

arr = [
    1,
    2,
    3
];


str = "multiline
dfsakfalf
"




```

# Match

```python

matchExample(v) =
    v match (
        == 1 -> 1,
        == "a string" -> _,
        == -2 -> 2
    )
    + 2;

```

# Rich enum

```python

|   A(
        pub test = "fsldkjf";
    );

|   B(
        val = false;
    );

```

# is syntax

is function

is string

is Type

# lambdas

```python

test() =
    arr map (_[0] + 2)

```


# stores / async

```python

test: $string = "df"

test() *string =
    arr map (_[0] + 2)

```
