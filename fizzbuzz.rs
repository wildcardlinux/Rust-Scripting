fn main () {
   for n in 1..101{
    if n % 15 == 0 {
        println!("fizzbuzz");
        sleep(Duration::from_millis(500));
    }
     else if n % 3 == 0 {
        println!("fizz");
        sleep(Duration::from_millis(500));
     }
     else if n % 5 == 0 {
        println!("buzz");
        sleep(Duration::from_millis(500));
     }
     else {
        println!("{}", n);
     }
   }
}
