fn main() {
  // format is just a format string assum we are saving it as print in a variable


  let a = 20;
  let b  = 30;
  let m = format!("Sum is {}",a+b);


  println!("{}",m);
   // vector .iter and .iter_mut


   let mut v  = vec![1,4,6,8];
    v.push(9);

   for x in &v{
    println!("{}",x);
   }

   //same as .iter()
   for  x in v.iter(){
    println!("{}",x)
   }

   //.iter_mut(). for mutate every element in vec during iteration


   for x in v.iter_mut(){
      *x += 1;
       println!("{}",x)
   }
  // v.len() give how many element stored
  
    println!("Length : {}", v.len());
  //v.capacity() give how many element can store when we push element capacity increse usesx2

  println!("Capacity : {}", v.capacity());
  

}
