struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

pub fn debug_test(){
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    println!("{:?}", DebugPrintable(32))
}
