
const ThreeHoursInSeconds: u32 = 60*60*3;
fn main() {
   let mut x = 5;
   println!("the value of x is {}", x);
   x = 6;
   println!("the value of x is {}", x);

   //let guess = "42".trim().parse().expect("Not a number");
   let guess: i32 = "42".trim().parse().expect("Not a number");

   let tup: (i32, f64, u8) = (500, 6.4, 1);

   let a = tup.0;//first 
   let b = tup.1;

   let array = [1,2345,6,7,3,2];
   let b: [i32; 5] = [3; 5];  // shorthand for [3, 3, 3, 3, 3]

   let f = array[0];


   let cond = true; 

  // let number = if cond {5} else {"six"}; // would not work as "six" is string
   let mut count = 1;
   'outerloop: loop{
    println!("count = {}", count);
    let mut  remaining = 10; 
    
    loop{
        println!("remaining = {}", remaining);
        if remaining < 9
          { break; }
        if count > 2 
          { break 'outerloop;} //this will break out the outer loop
        remaining -= 1;
    }
    count += 1;
   }

   let mut counter = 0;

   let result = loop{
    counter += 1;
    if counter == 9
    {
        break counter*2; //will return this ....would be the value of result
    }
   };

   for num in (1..4).rev() //loop prints in rev order
   {
    println!("{}!", num);
   }
}

//you know how we use fns 

fn five() ->  i32 {
    5 // will directly return 5
}
