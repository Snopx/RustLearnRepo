pub fn learn_type() {
    let mut a: i32 = 32;
    println!("before shadowing a ={}", a);
    a = 3;
    println!("overwritten a={}", a);
    let arr: [i32; 3] = [1, 2, 3];
    for elem in arr.iter() {
        println!("{}", elem);
    }
    print_arr(arr);

    //method: parse() ;两种使用方法 let x:type 声明时带参数；通过方法 prase::<type>() 返回相应类型
    let num: i32 = "32".parse().expect("32 is not a num");
    println!("string '32' parse to i32,and value={}", num);
    let num_float = "32.32"
        .parse::<f32>()
        .expect("string '32.32' parse to f32 err");
    println!("string '32.32' prase to f32,and the value={}", num_float);

    /* 整数类型 i32（i=>integer） u32(无符号,unsigned integier)
    i8,i16,i32,i64,i128,isize(arch,根据计算机位数 架构，使用场景 对某种集合进行索引操作)
    u8,u16,u32,u64,u128,usize(arch,根据计算机位数 架构)
    整数字面值： decimal(10):95_333 ; hex(0x:16):0xff ; Octal(0o:8):0o77; Binary(0b):0b1111_0101; Byte(u8 only ,b开头): b'A'; 除了byte类型 都允许使用类型后缀
    整数默认值：i32，即使是再 64位系统计算机 总体速度很快
    整数溢出：u8:0~255;调试模式下编译，Rust会检查整数溢出，如果发生溢出会 panic；
                     发布模式下(--release)编译，Rust不检查可能导致panic的溢出，如果发生溢出将发生”环绕“操作：如 256=>0,257=>1 即从头开始计算溢出部分（不会发生panic）；
    */

    /* 浮点类型  f32 单精度； f62 双精度
       默认类型 f64,现代CPU上 f64与 f32的速度差不多且精度更高；
    */

    let _f1 = 3.0; //f64
    let _f2 = 3.0f32; //f32

    //数值操作 只能同类型进项操作
    let _addition = 5 + 10;
    let _subtraction = 10 - 5;
    let _multiplication = 4.0 * 1.1;
    let _division = 3.0 / 2.1;

    // 布尔类型 bool  一个字节大小
    let _is_alive = true;

    //字符类型 char 占用 4个字节 unicode标量值，可以表示ASCII多的多的字符内容：拼音，中日韩文，零长度空白支付，emoji表情等  u+0000~u+D7FF;U+E000~U+10FFFF
    let _char_a = 'A';
    let emoji = '😂';
    println!("{}", emoji);

    //复合类型； 元组 tuple ; 数组
    //tuple：将多个类型的多个值，放入一个类型里；长度是固定的，一旦声明就无法改变；

    //创建 tuple
    let tup: (i32, f64, u8) = (32, 64.64, 22);
    println!("---------tuple---------");
    println!("tuple={},{},{}", tup.0, tup.1, tup.2); //索引号访问
    println!("---------tuple---------");

    //获取tuple 的元素值 destructor 解构
    let (x, y, z) = tup;
    println!("---------tuple destructor---------");
    println!("x={},y={},z={}", x, y, z);
    println!("---------tuple destructor---------");

    // 数组：数组也可以将多个值放在一个类型里，数组中每个元素的类型必须相同，数组的长度是固定的
    let _arr_int32 = [1, 2, 3, 4, 5];
    //数组的用处 如果想让数据存放在 stack （栈，（箱子，后进先出））上 而不是 heap （堆：堆是一种经过排序的树形数据结构，每个结点都有一个值。常为二叉堆，堆的特点是根结点的值最小（或最大），且根结点的两个子树也是一个堆。由于堆的这个特性，常用来实现优先队列，堆的存取是随意，这就如同我们在图书馆的书架上取书，虽然书的摆放是有顺序的，但是我们想取任意一本时不必像栈一样，先取出前面所有的书，书架这种机制不同于箱子，我们可以直接取出我们想要的书。）上，或者想保证有固定数量的元素，这时使用数组更有好处； 实际上rust中 常使用Vector ，数组没有Vector灵活，Vector 长度可以改变

    let arr_qipa = [3; 3]; //=> let _arr_qipa = [3,3,3];
    println!("arr_qipa[0] ={} ", arr_qipa[0]);
    print_arr(arr_qipa);
    //数组是 stack 上分配的单个块的内存,连续的
}

// [type; len]; the len is one of the type for arr; 长度也是数组类型的一部分
fn print_arr(arr: [i32; 3]) {
    println!("----------");
    for elem in &arr {
        println!("{}", elem);
    }
}
