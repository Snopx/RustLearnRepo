use std::u32;

pub fn struct_demo() {
    let w = 30;
    let h = 30;
    let area = area(w, h);
    println!("area1 is :{}", area);
    let dim = (20, 20);
    let area2 = area2(dim);
    println!("area2 is :{}", area2);

    let rec = Rec {
        width: 10,
        height: 3,
    };
    println!("area3 is :{}", area3(&rec));

    println!("rec is {:?}", rec);

    println!("rec is {:#?}", rec);

    println!("impl rec area() is {}", rec.area());

    let ctor = Rec::ctor(10, 20);
    println!("impl rec ctor() is {:#?}", ctor);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area3(rec: &Rec) -> u32 {
    rec.width * rec.height
}
#[derive(Debug)]
struct Rec {
    width: u32,
    height: u32,
}
// 实现Rec 方法
impl Rec {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn ctor(width: u32, height: u32) -> Rec {
        Rec {
            width: width,
            height: height,
        }
    }
}
