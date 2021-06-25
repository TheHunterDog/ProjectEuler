use core::num;

fn main() {

  let mut numbers = vec![1,2];
  let mut sum = 3;
  let mut sumeven = 2;
  let mut sumodd = 1;
  let limit = 0;
  
  let mut it = 0;
  println!("limit: {}",limit);
  loop {
    it+=1;
    println!("Iteration {}",it);
      // if numbers.len() != limit{
        let length = numbers.len() -1;
        // println!("{}",length);
        let one = numbers.get(length);
        let two = numbers.get(length - 1);
        match one {
          Some(x) => match two {
              Some(y) => {
                println!("Adding {} + {}",x,y);
                let ans = x+y;
                if ans > 4000000{
                  break;
                }
                numbers.push(ans);
                sum += ans;
                if ans % 2 == 0 {
                  sumeven += ans;
                }
                else{
                  sumodd+=ans;
                }
                
              }
              None=> break
          }
          None => break
        }
      // }
      // else{
      //   break
      // }
  }
  println!("sum:{}",sum);
  println!("sumodd:{}",sumodd);
  println!("sumeven:{}",sumeven);
  println!("numbers:{:?}",numbers);
}
