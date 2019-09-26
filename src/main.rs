//use std::io ;

// //function calculat the sum of the given number:
// fn add(x:i32, y:i32)->i32{
//     let sum = x+y;
//     sum
// }

// //function calculat the square of the given number:
// fn squr(x:i32)->i32{
//     let sq=x*x;
//     sq
// }

////function calculat the qube of the given number:
// fn qub(x:i32)->i32{
//     let qub=x*x*x;
//     qub
// }

// //function calculate the square and qube both of a given number and retun
// //each through tuple call
// fn sq_and_qub(x:i32)->(i32,i32){
//    //(x*x,x*x*x)    //shortcut mehod
//     let s=x*x;
//     let q=x*x*x;
//     (s,q)
// }

// //function return maxiumum value between two
// fn max(x:i32, y:i32)->i32{
//     if (x>y){
//         x
//     }
//     else {
//         y
//     }
// }

// //function take full name from user and store into tuple and print
// fn full_name()->(String,String){
//     println!("Enter First name");
//     let mut f_name =String::new();
//     io::stdin().read_line(&mut f_name);
//     let f_name:String = f_name.trim().parse().unwrap();

//     println!("Enter last name");
//     let mut l_name =String::new();
//     io::stdin().read_line(&mut l_name);
//     let l_name:String = l_name.trim().parse().unwrap();

//     (f_name,l_name) //return tuple
// }

////function return lagerst number out of array of multiple numbers
// fn greater(arry:&[i32])->i32{
//     let mut largest = arry[0];
//     for i in 1..arry.len(){
//         if largest < arry[i]{
//             largest = arry[i];
//         }
//     }
//     largest
// }

////function to calculate length of a String
// fn calculat_length(s:String)->(String,usize){
//     let lenght = s.len();
//     (s,lenght)
// }

////function return length of string with referencing method
// fn calculate_len(s:&String)->usize{
//     s.len()
// }
fn main() {
    // let mut in_01 = String::new() ;
    // io::stdin().read_line(&mut in_01);
    // let in_01:i32 = in_01.trim().parse().unwrap();          //for integer value // let input:String = input.trim().parse().unwrap();       //for strings

    // let mut in_02 = String::new() ;
    // io::stdin().read_line(&mut in_02);
    // let in_02:i32 = in_02.trim().parse().unwrap();

//println!("{}",add(in_01,in_02));

//let cal_result = sq_and_qub(in_01);
//println!("the number {} square is {}, qube is {} ",in_01,cal_result.0,cal_result.1);

//println!("the maximum nuber is {}",max(in_01,in_02));

// let result = full_name();
// println!("this is your name : {} {}",result.0,result.1);

// //let mut arr: [i32; 4] = [1, 2, 3, 4]; another way of defining array
// let num_array = [45,86,4,3,99,4,56,5,55,5];      
// println!("the lagest number in array is {} :",greater(&num_array));
////function ends

////ownership start
// let s =String::from("hello");
// //s.push_str(" world"); //append string to s
// let a1 = s.clone();
// println!("{}, {}",s,a1);

// let _s1= String::from("Osama");
// let (_s2, _len) = calculat_length(_s1);
// println!("the length of {} is {}",_s2,_len)

// //refrence start
// let s1 = String::from("Osama");
// let lenth = calculate_len(&s1);
// println!("{} {}",s1,lenth);

////empty vector declaration with pushing values
// let mut my_vector: Vec<i32> = Vec::new();
// my_vector.push(5);
// my_vector.push(9);
// my_vector.pop();
// my_vector.pop();
// println!("{:?}",my_vector);

////another way of declaring vector
// let my_vector_01: Vec<i32> = vec![2,5,3,4,6,9,8];//could also be held empty
// let vector_02  = vec!["Faheem","Ibad","Zain"];//for strings
// println!("{:?}",my_vector_01);
// println!("{:?}",vector_02);

//now accessing elements of vector have two ways
//1->By using refrence 2->By using get() (see later)
// let v: Vec<i32> = vec![4,6,3,8,9];
//// let third: &i32 = &v[2];
//// println!("{}",third);

// let mut total = 0;
// for i in &v {
//     total = total+i;
//     println!("{}",i);
// }
// println!("{}",total);
}
