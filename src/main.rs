mod fmt_test;
use logging::add;

fn main() {
    greet_world();
    fmt_test::debug_test();
}

fn greet_world(){
    let chinese  = "世界你好";
    let english  = "hello world";
    let mut known  = "known";

    let regions = [chinese, english, known];
    eprintln!("{:?}", regions);
    for r in regions.iter(){
        println!("{}",r)
    }
    known = "known2";
    eprint!("{:?}\n",known);
    
    print!("{}", add(3,2));

}
