// fn main() {
//     let rect = (50,40);

//      println!(
//      "the area of the rectangle is {} square pixels.",
//      area(rect)
//     );
// }
// fn area(dimension:(u32,u32))-> u32 {
//     dimension.0*dimension.1
// }

// struct Rectangle{
//      width:u32,
//      height:u32,
// }
// fn main(){
//      let rect1 = Rectangle{width:30,height:50};
//      println!("the area of the rectangle is {} square pixel.",
//      area(&rect1)
// );
// }
// fn area(rectangle:&Rectangle)->u32{
//     rectangle.width*rectangle.height
// }

// #[derive(Debug)]
//       struct Rectangle{
//        width: u32,
//       height: u32,
// }
// impl Rectangle {
//       fn area(&self)-> u32{
//       self.width * self.height
//      }
// }
// fn main(){
//      let rect1 = Rectangle{width:50,height:50};
//       println!(
//           "the area of the rectangle is {} square pixels.",
//           rect1.area()
//      );
// }
// bounding a variable with a string
// fn main(){
//      let  x ="Ali Abdullah Raheel Khan Akash";

//      let y = {
//           let x =5;
//           x+4
//      }; 
//      println!("the value of the x and y is: {} {}",x,y);
//      let z = x.len();
//      println!("the length of the variable is:{}",z);
// }
//making a variable mutable and giving another value to that variable
// fn main(){
//      let mut x = "ali abdullah raheel khan akash";
//      println!("the of the best programmer in this universe is:{}",x);
//      x ="ali asadullah shaoor khan akash";
//      println!("the name of his younger brother is:{}",x);
// }

// fn main(){l;cin cxxxxxxccckip==
//     println!("Hello World");
//     another_function();

    
// }
// fn another_function(){
//     println!("another_function");
// }

// fn main(){
//     let x = 3;
//      if  x != 5 {
//      println!("the condition was true");
// }


// }

// fn main(){
//     let mut counter = 0;
//     let result = loop{
//         counter +=1;
//         if counter==10{
//         break counter*2;
//         }

//     };
//     println!("the result of the process is:{}",result);
// }

// fn main(){
// let mut number =3;
// while  number !=0{
//     println!("{}!",number);
//      number -= 1;
// }
// println!("LIFTOFF!!!");
// }

// fn main(){
//     let a =[10,20,30,40,50];
//     for element in a.iter(){
//         println!("the value from a is:{}",element);
//     }
//} 

// fn main(){
//     let s1 = String::from("Hello");
//     let len = calculate_length(&s1);
//     println!("the length of '{}' is {}.",s1,len);
// }
// fn calculate_length(s: &String)-> usize{
//     s.len()
// }

// fn main(){
//     let  mut s =String::from("Hello,World");
//     let  _word = first_word(&s);
//     println!("the first word of the string is {}",_word);
//     s.clear(); 
// }
// fn first_word(s: &String)-> usize {
//     let bytes= s.as_bytes();
//     for(i,&item) in bytes.iter().enumerate(){
//         if item == b' ' {
//             return i;
//         }

//     }
//     s.len()
// }


fn main(){
    const PI = 3.14;
    println!("{}",PI);
}

