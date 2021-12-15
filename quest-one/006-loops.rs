fn main(){
    let mut x=2;
    loop{
        if x>1000 {
        break;}
        x*=2;
        println!("{}",x);
    }

     let mut y=x;
     while y>0{
         println!("{}",y);
         y/=2;
     } 
    // 0 to 9
    for x in 0..10{  
        println!("{}",x);
    }
    // 0 to 10
    for x in 0..=10{  
        println!("{}",x);
    }
    let y=[1,2,3,4];
    for val in y{
        println!("{}",val);
    }
}