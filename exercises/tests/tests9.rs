// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
/*Rust 非常能够与 C/C++ 和其他静态编译的接口共享 FFI 接口。
语言，它甚至可以在代码本身内链接！它通过外部
块，就像下面的代码一样。 */

// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
/*“extern”关键字后面的短字符串指示外部导入的 ABI。
功能将随之而来。在本练习中，使用了“Rust”，而存在其他变体，例如
“C”代表标准C ABI，“stdcall”代表Windows ABI。 */

// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
/*外部导入的函数在外部块中声明，分号
标记签名的末尾，而不是大括号。某些属性可以应用于这些
用于修改链接行为的函数声明，例如 #[link_name = “..”] 到
修改实际符号名称。 */

// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
/*如果要将符号导出到链接环境，“extern”关键字可以
还要在具有相同 ABI 字符串注释的函数定义之前进行标记。默认 ABI
因为 Rust 函数的字面意思是“Rust”，所以如果你想链接到纯粹的 Rust 函数，
可以省略整个外部术语。 */

// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
/*默认情况下，Rust 会破坏符号，就像C++一样。要抑制此行为并使
那些可按名称寻址的函数，可以应用属性 #[no_mangle]。 */

// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
/*在本练习中，您的任务是使测试用例能够调用 “my_demo_function”
模块福。“my_demo_function_alias”是“my_demo_function”的别名，因此两者
测试用例中的代码行应调用相同的函数。
// */
// You should NOT modify any existing code except for adding two lines of attributes.



extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    #[no_mangle]
    // No `extern` equals `extern "Rust"`.
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
