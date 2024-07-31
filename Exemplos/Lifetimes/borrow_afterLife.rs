
fn main() {
    let shulambs;
    {
        let  var = String::from("to aq no escopo B");
        shulambs = &var;
    }
    // erro pois o borrow vive mais que a variável owner do valor
    println!("{}", shulambs);
  


}