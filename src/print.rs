pub fn run(){
    //Print to console
    println!("Hello from print.rs file");

    //Basic formatting
    println!("{} is from {}","shyam","mass");

    //positional arguments
    println!("{0} is from {1} and he likes to {2}","shyam","mars","code");

    //Named arguments
    println!("{name} likes to play {activity}",name="shyam",activity="cricket");

    //placeholder traits
    println!("Binary :{:b}  Hex :{:x}  Octal: {:o}",10,10,10);

    //placeholder for debug trait => tuples
    println!("{:?}",(12,true,"Hello"));

    //Basic Math
    println!("{} + {} = {}",10,20,10+20);


}