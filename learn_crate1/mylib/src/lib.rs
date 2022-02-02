//! My Crate
//! 
//! 'my_crate' is a collection of utilites to make perfoming certain calcutions more convenient
//! 
/// add on to the number given
///  
/// #Eexample 
/// ```
/// let five = 5;
/// 
/// assert_eq!(6,mylib::add_one(5));
/// ```

pub fn add_one(x:i32) ->i32{
x +1
}



// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
