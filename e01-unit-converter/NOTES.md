# Convert CLI Tool: Things learned while building this project

- `std::env::args()`: Returns an _iterator_ with `String`s as its items.
- `.next()`: Calling it on an _iterator_ produces an `Option<String>` which might contain an argument or not since it has two options:

1. `Some(String)`: The argument exists, and the `String` is wrapped safely inside.
2. `None`: There is no argument.

- We can use `unwrap` to open the box(`Option<String>`) and get the data on which we can perform desired operations but panics(crashes the program) if it's `None`.

- Putting **&** infront of a `String` creates a reference of type `&String`. Rust then automatically converts(_coerces_) the `&String` into a `&str` wherever a function expects a `&str`.

- **How {:.2} rounds**: does not apply in the conversions--for example, 0.125 -> 0.12 and 2.675 -> 2.67.

This is simply because 0.125 can be fully represented(without any loss in precision) in binary form, 0.12 is also chosen instead of 0.13 because Rust chooses the **result whose last retained digit is even only when the value is exactly _halfway_**--for example: 0.125 -> 0.12(not 0.13) and 0.875 -> 0.88(not 0.87)

But 2.675 cannot and so its binary representation is just an approximation and when converted to decimal it's slightly higher than the stored value so it rounds down to 2.67.
