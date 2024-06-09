fn pow(base: i32, exp: i32) -> i32 {
    if exp == 0 {
        return 1;
    }
    let mut result = base;
    for _ in 1..exp {
        result *= base;
    }
    return result;
}

fn pow10(exp: i32) -> i32 {
    return pow(10, exp);
}

fn pow10f(exp: i32) -> f32 {
    return pow(10, exp) as f32;
}

const MASK_SIGN: i32 = 0x8000;
const MASK_INTEGER: i32 = 0x03FF;
const MASK_DECIMAL: i32 = 0x3C00;

fn main() {
    println!("================Encode================");
    let mut value = -36.5_f32;

    let mut result = 0_i32;

    if value < 0f32 {
        result |= ((1 << 15) & MASK_SIGN);
        value = value.abs()
    }
    println!("Sign: {:?}", (result & MASK_SIGN) == MASK_SIGN);

    let integer_part = value as i32;
    println!("Integer: {:?}", integer_part);

    let decimal_part = ((value * pow10f(1)) - (integer_part as f32) * pow10f(1)) as i32;
    println!("Decimal: {:?}", decimal_part);

    result |= (integer_part & MASK_INTEGER);
    result |= (decimal_part << 10 & MASK_DECIMAL);

    println!("Result: {:?}, {:x}, {:016b}", result, result, result);

    println!("================Decode================");
    let signed = (result & MASK_SIGN) == MASK_SIGN;
    println!("Sign: {:?}", signed);

    let integer = (result & MASK_INTEGER) as f32;
    println!("Integer: {:?}", integer);

    let decimal = ((result & MASK_DECIMAL) >> 10) as f32 / pow10f(1);
    println!("DECIMAL: {:?}", decimal);

    let value = integer + decimal;

    let value = if signed {
        -value
    } else {
        value
    };
    println!("Result: {:?}", value);
}
