

// 代码组织主要包括
// 那些细节可以暴露，哪些细节是私有的
// 作用域内哪些名称有效
// 模块系统
// Package ：cargo 特性，构建、测试、共享crate
// carge(单元包)： 一个模块数，它可以产生一个library或可执行文件
// Module（模块）、use： 控制代码组织，作用域，私有路径
// path: 为struct function module 等命名的方式


// crate的类型：
//      library
//      binary

// Crate Root:
// -是源代码文件
// Rust编译器从这里开始，组成你的Crate的根module

// 一个Package
// 包含一个Cargo.toml，他描述了如何构建这些Crates
// 只能包含0~1个 library crate
// 可以包含任意数量的 binary crate
// 但必须至少包含一个 crate （library,binary）