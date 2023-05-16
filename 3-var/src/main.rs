// 1. Rust 变量默认是不可变的

//  fn main() {
//     let x = 5;
//     println!("x is {}",x);
//     // x = 6 // error: could not compile `var` due to previous error
//     // println!("x is {}",x); 
// }

// 2. Rust变量可变
// fn main() {
//   let mut x = 5;
//   println!("x is {}",x);
//   x = 6;
//   println!("x is {}",x); 
// }

// 3. 声明忽略未使用的变量,变量名前缀加上下划线
// fn main() {
//   let _x = 5;
// }

//  4. 解构赋值
// fn main(){
//   let (a, mut b ) = (true,false);

//   println!("a = {:?},b={:?}",a, b);
//   b = true;
//   println!("a = {:?},b={:?}",a, b);
// }


// 5. 常量
// fn main(){
//   const MAX_POINTS: u32 = 100_000;
// }

// 6. 变量作用域(变量遮蔽)
fn main() {
  let x = 5;
  // 在main函数的作用域内对之前的x进行遮蔽
  let x = x + 1;

  {
      // 在当前的花括号作用域内，对之前的x进行遮蔽
      let x = x * 2;
      println!("The value of x in the inner scope is: {}", x); // 12
  }

  println!("The value of x is: {}", x); // 6
}
