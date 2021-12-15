fn main(){
    let x=2;
    match x {
        1 => println!("VALUE OF X IS {}",x),
        _ => println!("NO MATCH"),
    }

    let a=true;
    let b=false;
     match (a, b){
         (true,true) => println!("both true"),
         (false,false) => println!("both false"),
         (true,false) =>println!("a true"),
         _ => println!("NO CASE")
     }
}