// 梯形公式
fn fn_tx<F>(x0:f64,x1:f64,fx:F)->f64
    where F:Fn(f64)->f64,
{
    let mut re = 0.5*(x1 - x0);
    re *= fx(x0) + fx(x1);
    re
}

// Simposon公式
fn fn_sim<F>(x0:f64,x1:f64,fx:F)->f64
    where F:Fn(f64)->f64,
{
    let mut re = (x1 - x0)/6.0;
    re *= fx(x0) + fx(x1) + 4.0*fx((x0 + x1)/2.0);
    re
}

// (1)
fn f1(x:f64)->f64
{
    let re = x.powi(4);
    re
}
// (2)
fn f2(x:f64)->f64
{
    let re = 2.0/(x-4.0);
    re
}
// (3)
fn f3(x:f64)->f64
{
    let mut re = x.powi(2);
    re *= x.ln();
    re
}
// (4)
fn f4(x:f64)->f64
{
    let mut re = x.powi(2);
    re *= (-x).exp();
    re 
}

fn main() {
    println!("(1):梯形公式结果为    {},     Simposon公式结果为  {}",fn_tx(0.5, 1.0, f1),fn_sim(0.5, 1.0, f1));
    println!("(2):梯形公式结果为    {},     Simposon公式结果为  {}",fn_tx(0.0, 0.5, f2),fn_sim(0.0, 0.5, f2));
    println!("(3):梯形公式结果为    {},     Simposon公式结果为  {}",fn_tx(1.0,1.5, f3),fn_sim(1.0,1.5, f3));
    println!("(2):梯形公式结果为    {},     Simposon公式结果为  {}",fn_tx(0.0, 1.0, f4),fn_sim(0.0, 1.0, f4));
}
