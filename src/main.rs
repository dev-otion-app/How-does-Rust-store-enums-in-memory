use std::mem;
use std::slice;

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
    A(u64),
    B(Box<String>)
}

enum SixthEx{
    A,
    B([u8;4],bool),
    C
}

enum SeventhEx{
    A,
    B([u16;4],bool),
    C
}

fn main() {

    print_bytes(&FirstEx::A); // [0]
    print_bytes(&FirstEx::B); // [1]

    print_bytes(&SecondEx::A); // [0,0,0,0]
    print_bytes(&SecondEx::B(13)); // [1,0,13,0]

    print_bytes(&ThirdEx::A(3)); // [0,0,0,0,0,0,0,0]
    print_bytes(&ThirdEx::B([1,2,3,4,5,6])); // [1,1,2,3,4,5,6,0]

    print_bytes(&FourthEx::A(4)); // [0] * 8 + [4]+[0]*7 + [0]*8
    print_bytes(&FourthEx::B(String::from("Dev-otion"))); // [something]*8 + [9]+[0]*7 + [9]+[0]*7

    print_bytes(&FifthEx::A(4)); // [0]*8 + [4]+[0]*7
    print_bytes(&FifthEx::B(Box::new(String::from("Dev-otion")))); // [1]+[0]*7 + [something]*8

    print_bytes(&SixthEx::A); // [2,0,0,0,0]
    print_bytes(&SixthEx::B([8,2,3,4],true)); // [1,8,2,3,4]
    print_bytes(&SixthEx::C); // [4,0,0,0,0]

    print_bytes(&SeventhEx::A); // [0]*8
    print_bytes(&SeventhEx::B([8,2,3,4],false)); // [1, 1, 8, 0, 2, 0, 3, 0, 4, 0]
    print_bytes(&SeventhEx::C); // [2] + [0]*7

}

fn print_bytes<T>(input: &T) {
    let size = mem::size_of::<T>();
    let bytes = unsafe {
        slice::from_raw_parts(
            input as *const T as *const u8,
            size
        )
    };
    println!("{:?}", bytes);
}
