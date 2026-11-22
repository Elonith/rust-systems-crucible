# Convert CLI Tool: Things learned while building this project

- `std::env::args()`: Returns an _iterator_ with `String`s as its items.
- `.next()`: Calling it on an _iterator_ produces an `Option<String>` which might contain an argument or not since it has two options:

1. `Some(String)`: The argument exists, and the `String` is wrapped safely inside.
2. `None`: There is no argument.

- We can use `unwrap` to open the box(`Option<String>`) and get the data on which we can perform desired operations but panics(crashes the program) if it's `None`.

- Putting **&** infront of a `String` creates a reference of type `&String`. Rust then automatically converts(_coerces_) the `&String` into a `&str` wherever a function expects a `&str`.
