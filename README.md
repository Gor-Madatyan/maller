This is a small Rust library  that provides a Maller structure, which allows you to call closures by key.

# [Documentation][1]

# Usage\`<br>
Add this to your Cargo.toml file:

```
[dependencies]
maller = "0.1"
```
## or use
`cargo add maller`<br><br>

Then, you can use the `Maller` and `Input` structures in your Rust code<br>
The `input` and `new_input` functions from `utils` allow you to create an input closure. More details in the [documentation][1].

## Example

```
use maller::{input, Maller};

fn main() {
    let mut con=0;

    let c1=|x|{con+=1;2};
    let c2=|x|3;

    let mut maller=Maller::from_iter(
        [(898, input(c1)),
         (500, input(c2))]
    );

    assert_eq!(maller.call(&898).unwrap(),2);
}
```



## License
This code is licensed under the `"MIT OR Unlicense"` License. 

[1]:(https://docs.rs/maller)