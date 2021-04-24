use std::u32;

/*
iterator trait 实现于 std
pub trait Iterator{
    type Item;
    fn next(&mut self)->Optional<Self:Item>{
        ...
    }
}
 type Item 与 Self::Item 定义了与此 trait 关联的类型；
*/
#[cfg(test)]
mod tests_iter {
    #[test]
    fn test_iter() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        for item in &v1 {
            println!("got:{}", item);
        }
        for item in v1_iter {
            // for will take the ownership of v1_iter
            // and make it mutalbe
            println!("got:{}", item);
        }
    }

    // the differences of methods about iterator
    // iter() 在不可变引用上创建迭代器
    // into_iter() 创建的迭代器会活得所有权
    // iter_mut() 迭代可变的引用
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1)); //next() will consume the value of iter
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));

        let it = v1.into_iter();
        for i in it {
            println!("{}", i);
        }
        let mut v2 = vec![1, 2, 3];
        let mutit = v2.iter_mut();
        for mi in mutit {
            *mi += 1;
        }
        println!("{:?}", v2);
    }

    // std 中 iterator trait 有一些默认的实现方法
    // 其中有一些方法会调用 next 方法
    // 所以 实现 iterator trait 必须实现 next 方法
    // 这样 所有 调用 next 的方法叫做 “消耗型适配器”
    // 例如： sum 方法 该方法通过反复调用 next 遍历所有元素

    #[test]
    fn iter_sum() {
        let v1 = vec![1, 2, 3];
        let sum: i32 = v1.iter().sum();
        assert_eq!(sum, 6)
    }

    // 产生其他迭代器的方法
    // 定义在 iterator trait 上的另一些方法叫做 “迭代器适配器”
    // -将迭代器转换为不同种类的迭代器
    // 可以通过链式调用使用多个迭代器适配器来执行复杂的操作， 这种调用可读性高
    // 例如 map  ; 接收一个闭包作用于每个元素
    // 迭代器是惰性加载的（lazy) 如果没有消耗他，那么什么都不会做
    #[test]
    fn iter_map() {
        let v1 = vec![1, 2, 3];
        let c: Vec<_> = v1.iter().map(|x| x + 1).collect(); // collect方法就是一个消耗型适配器，将结果收集到一个集合类型中
        assert_eq!(c, vec![2, 3, 4]);
    }

    // 迭代器闭包捕获环境
    // filter 方法：迭代器适配器
    // 接收一个闭包
    // 这个闭包在遍历迭代器的每个元素时，返回一个bool
    // 如果闭包返回true，当前元素将会包换在 filter产生的 迭代器中
    #[test]
    fn iter_filter() {
        let v1 = vec![1, 2, 3, 4, 5];
        let fi: Vec<_> = v1.into_iter().filter(|x| x > &3).collect();
        println!("{:?}", fi);
        assert_eq!(fi, vec![4, 5]);
    }

    //

    use super::*;
    #[test]
    fn test_next_Counter() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn test_mul_Counter() {
        let mut counter = Counter::new();
        let sum: u32 = counter
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        println!("{}", sum);
        assert_eq!(sum, 18);
    }
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
// 实现迭代器
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// zero-cost abstraction  零开销抽象
// 使用 抽象时不会引入额外的运行时开销
// 所以使用 迭代器 比使用 for循环 更快！！
// 编译器 会将 代码展开 如果迭代12次 就产生12行相同代码（汇编） 消除循环控制语句带来的性能开销
