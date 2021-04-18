use std::{collections::HashMap, thread, usize};
use std::{hash::Hash, time::Duration};

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
#[test]
fn test_closure() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    assert_eq!(3, simulated_expensive_calculation(3));
}

#[test]
fn test_type_inference_annotaion() {
    let intensity = 30;
    let random_number = 10;
    let expensive_closure = |num: u32| -> u32 {
        // 闭包 匿名函数
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity)); // 这种方法会执行两次该方法，不太好，使用泛型闭包，缓存返回值
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| x + 1; // 使用类型推断 该闭包函数必须被调用
    // let add_one_v4 = |x| x + 1; // 切被调用后 该闭包函数类型就被确定
}

// 每个闭包的实例都有自己唯一的匿名类型，即使两个闭包签名完全一样
// 所有的闭包都实现了 以下 trait 之一
// Fn
// FnMut
// FnOnce
struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
{
    calculation: T,
    map: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: Eq + Hash + Copy,
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }
 
    pub fn value(&mut self, arg: K) -> &V {
        self.map.entry(arg).or_insert((self.calculation)(arg))
    }
}

#[test]
fn generic_closure() {
    let mut expensive_closure1 = Cacher::new(|t1: &str| -> usize { t1.len() });
    let intensity = 20;
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure1.value("intensity"));
        println!("Next, do {} situps!", expensive_closure1.value("123")); //
    } else {
        if 100 == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure1.value("123213")
            );
        }
    }
}
