fn main() {
    println!("If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

    Find the sum of all the multiples of 3 or 5 below 1000.");

    let mut numbers =  vec![];
    let limit = 1000;
    let mut sum:i64 = 0;
    let multiples = [3,5];
      for table in multiples {
        let mut it = 0;
        let mut sumTable = 0;
        loop {
          it+=1;
          let res = table*it;

          if res < limit {
            if !numbers.contains(&res){
            numbers.push(res);
            sum+=res;
            sumTable+=res;
            }
          }
          else{
            // println!("{:?}",numbers);
            println!("ans:{}",sum);
            println!("sumTable:{}",sumTable);

            break;
      
          }
        }
      }
      // println!("{:?}",numbers);
      println!("{}",sum);

}
