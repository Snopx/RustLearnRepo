pub mod generic_learn {
    use std::fmt::Debug;

    pub fn sort(list: &[i32]) -> i32 {
        let mut largest = list[0];
        for &m in list {
            if m > largest {
                largest = m;
            }
        }
        largest
    }

    pub fn sort_generic<T>(list: &[T]) -> &T
    where
        T: PartialOrd, //+ Copy, // 要返回对应类型 而不是&T 需要copy trait
    {
        let mut largest = &list[0];
        for value in list {
            if value > &largest {
                largest = value;
            }
        }
        largest
    }

    #[derive(Debug)]
    pub struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        //rust 编译时 会更具具体类型 单态化，即 会转换为对应类型，所以泛型的速度不会减慢
        pub fn new_point(x: T, y: T) -> Point<T> {
            Point { x, y }
        }
        pub fn get_x(&self) -> &T {
            &self.x
        }
        pub fn get_y(&self) -> &T {
            &self.y
        }
    }
}
