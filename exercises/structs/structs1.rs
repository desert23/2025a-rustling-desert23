// structs1.rs
//
// Address all the TODOs to make the tests pass!
//
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a
// hint.

// structs1.rs

// structs1.rs

// 定义一个经典结构体
struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

// 定义一个元组结构体
struct ColorTupleStruct(u8, u8, u8);

// 定义一个单位结构体
#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // 初始化 green 为 ColorClassicStruct 实例
        let green = ColorClassicStruct {
            red: 0,
            green: 255,
            blue: 0,
        };

        // 进行测试
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // 初始化 green 为 ColorTupleStruct 实例
        let green = ColorTupleStruct(0, 255, 0);

        // 进行测试
        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // 初始化 unit_like_struct 为 UnitLikeStruct 实例
        let unit_like_struct = UnitLikeStruct;

        // 进行测试
        let message = format!("{:?}s are fun!", unit_like_struct);
        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
