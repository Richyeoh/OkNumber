# Design of Number

## Number(2Byte)

<table>
    <tr>
        <td>7</td>
        <td>6</td>
        <td>5</td>
        <td>4</td>
        <td>3</td>
        <td>2</td>
        <td>1</td>
        <td>0</td>
        <td>7</td>
        <td>6</td>
        <td>5</td>
        <td>4</td>
        <td>3</td>
        <td>2</td>
        <td>1</td>
        <td>0</td>
    </tr>
    <tr>
    <td colspan="1" style="text-align: center">SIGN</td>
    <td colspan="1" style="text-align: center">RESERVED</td>
    <td colspan="4" style="text-align: center">DECIMAL</td>
    <td colspan="10" style="text-align: center">INTEGER</td>
    </tr>
</table>

`Sign mask:` 0x8000

`Decimal mask:` 0x3C00

`Integer mask:` 0x03FF

U can find this an example
in [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=f7bf8ffd05106857d27b12ee5304594b)

## Encode example:

```rust
fn main() {
    let mut value = -36.5_f32;

    let mut result = 0_i32;
    if value < 0f32 {
        result |= ((1 << 15) & MASK_SIGN);
        value = value.abs()
    }

    let integer_part = value as i32;
    let decimal_part = ((value - integer_part as f32) * pow10f(1).round()) as i32;

    result |= (integer_part & MASK_INTEGER);
    result |= (decimal_part << 10 & MASK_DECIMAL);

    println!("Result: {:?}, {:x}, {:016b}", result, result, result);
}
```

``Result: 37924, 9424, 1001010000100100``

## Decode example:

```rust
fn main() {
    let result = 1001010000100100;

    let signed = (result & MASK_SIGN) == MASK_SIGN;
    let integer = (result & MASK_INTEGER) as f32;
    let decimal = ((result & MASK_DECIMAL) >> 10) as f32 / pow10f(1);
    let value = integer + decimal;

    let value = if signed {
        -value
    } else {
        value
    };

    println!("Result: {:?}", value);
}

```

``Result: -36.5``
