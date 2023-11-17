use std::mem;

enum FirstEx{
    A,
    B
}

enum SecondEx{
    A,
    B(u16)
}

enum ThirdEx{
    A(u16),
    B([u8;6])
}

enum FourthEx{
    A(u64),
    B(String)
} 

enum FifthEx{
    A,
    B([u8;4],bool),
    C
}

enum SixthEx{
    A,
    B([u16;4],bool),
    C
}

fn main() {

    println!("{:?}", get_bytes(&FirstEx::A)); // [0]
    println!("{:?}", get_bytes(&FirstEx::B)); // [1]

    println!("{:?}", get_bytes(&SecondEx::A)); // [0,0,0,0]
    println!("{:?}", get_bytes(&SecondEx::B(13))); // [1,0,13,0]

    println!("{:?}", get_bytes(&ThirdEx::A(3))); // [0,0,0,0,0,0,0,0]
    println!("{:?}", get_bytes(&ThirdEx::B([1,2,3,4,5,6]))); // [1,1,2,3,4,5,6,0]

    println!("{:?}", get_bytes(&FourthEx::A(4))); // [0] * 8 + [4]+[0]*7 + [0]*8
    println!("{:?}", get_bytes(&FourthEx::B(String::from("Dev-otion")))); // [something]*8 + [9]+[0]*7 + [9]+[0]*7

    println!("{:?}", get_bytes(&FifthEx::A)); // [2,0,0,0,0]
    println!("{:?}", get_bytes(&FifthEx::B([8,2,3,4],true))); // [1,8,2,3,4]
    println!("{:?}", get_bytes(&FifthEx::C)); // [4,0,0,0,0]

    println!("{:?}", get_bytes(&SixthEx::A)); // [0]*8
    println!("{:?}", get_bytes(&SixthEx::B([8,2,3,4],false))); // [1, 1, 8, 0, 2, 0, 3, 0, 4, 0]
    println!("{:?}", get_bytes(&SixthEx::C)); // [2] + [0]*7

}

fn get_bytes<T>(input: &T) -> &[u8] {
    let size = mem::size_of::<T>();
    let bytes = unsafe {
        std::slice::from_raw_parts(
            input as *const T as *const u8,
            size
        )
    };
    bytes
}
