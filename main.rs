fn main(){
    for x in 0..=1000000
    {
        let mut y = match(x % 3, x % 5, x % 6)
        {
            (0,0,_) => String::from("FizzBuzz"),
            (_,_,0) => String::from("Bazz"),
            (_,0,_) => String::from("Buzz"),
            (0,_,_) => String::from("Fizz"),
            (_,_,_) => x.to_string()
        };
        println!("{y}");
    }
}
