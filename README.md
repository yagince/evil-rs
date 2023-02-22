# Evil

Utility libraries in Rust.

## derive(Omit)

A new type can be automatically generated without the specified fields based on a specific type.
It is inspired by [Omit](https://www.typescriptlang.org/docs/handbook/utility-types.html#omittype-keys) of TypeScript.
``` rust
use evil::Omit;

#[derive(Omit, Debug)]
#[omit(NewHoge, id, derive(Debug, Clone))]
struct Hoge {
    pub id: u64,
    pub age: u64,
}
```

The above code generates the following code.

``` rust
#[derive(Debug, Clone))]
struct NewHoge {
    pub age: u64,
}
```
