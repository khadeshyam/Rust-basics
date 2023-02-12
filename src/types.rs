pub fn run() {
   // prefix it _ tells it should be unused

    //Default is "i32"
    let  x = 1;

    //Default is "f64"
    let  y = 2.5;

    //Add explicit type
    let  z: i64 = 47585959505050;

    //Find max size
    println!("Max i32: {}",std::i32::MAX);
    println!("Max i64: {}",std::i64::MAX);

    //Boolean
    let is_active :bool = true;

    //Get boolean from expression
    let is_greater:bool = 1 > 5;

    let a1:char = 'a';
    let face:char ='\u{1F600}';

    println!("{:?}",(x,y,z,is_active,is_greater,a1,face));




}
