//use crate::OperationWithVariant::Add;
fn main() {

    enum Result<T,E> {
        Ok(T),
        Err(E)
    }
   let  x = 3.4;
   let  y = 6.7;
   let sum = Add(x,y);
   
   match sum {
       Ok(value) => println!("the sum of {} and {} is {}",x,y,value),
       Err(error)=> println!("Error {}",error),
   }
   
   let diff=Subtract(x,y);
   match diff {
    Ok(value) => println!("the diff of {} and {} is {}",x,y,value),
    Err(error)=> println!("Error {}",error),
}
   let product=Mutiply(x,y);
   match product {
    Ok(value) => println!("the product of {} and {} is {}",x,y,value),
    Err(error)=> println!("Error {}",error),
}
   let Division=Divide(x,y);
   match Division {
    Ok(value) => println!("the ratio difference btw {} and {} is {}",x,y,value),
    Err(error)=> println!("Error {}",error),
}

 


}



enum OperationWithVariant{
    Add(f64,f64),
    Subtract(f64,f64),
    Mutiply(f64,f64),
    Divide(f64,f64),
}

fn calculate(calculate:OperationWithVariant){
    match calculate {
        OperationWithVariant::Add(x,y)=>
         println!("the sum of {},{} is",x,y),
        OperationWithVariant::Subtract(x,y  )=>
         println!("the differnce btw {} and {} is",x,y),
        OperationWithVariant::Mutiply(x, y)=>
         println!("the product of {} and {} is ",x,y),
        OperationWithVariant::Divide(x,y  )=>
         println!("the ratio difference between {} and {} is ",x,y),
    }
}
fn Add(x:f64,y:f64)->Result<f64, String>{
    if y<=0.0 {
            Err("Error".to_string())
    }else {
            Ok(x+y)
        }
}

    fn Subtract(x:f64,y:f64)->Result<f64, String>{
        if y<=0.0 {
            Err("Error".to_string())
        }else {
            Ok(x-y)
        }
    }

    fn Mutiply(x:f64,y:f64)->Result<f64, String>{
        if y==0.0 {
            Err("Error".to_string())
        }else {
            Ok(x*y)
        }
    }

    fn Divide(x:f64,y:f64)->Result<f64, String>{
        if y==0.0 {
            Err("Error".to_string())
        }else {
            Ok(x/y)
        }
    }
