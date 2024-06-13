
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

// 柯特斯公式
fn fn_cotes<F>(x0:f64,x1:f64,fx:F)->f64
    where F:Fn(f64)->f64,
{
    let mut re = (x1 - x0)/90.0;
    let n = (x1 - x0)/4.0;
    re *= 7.0*fx(x0) + 32.0*fx(x0 + n) + 12.0*fx(x0 + 2.0*n) + 32.0*fx(x0 + 3.0*n)+ 7.0*fx(x1);
    re
}


fn f(x:f64)->f64
{
    let re = x.exp();
    re
}

fn main() {
    println!("(1):梯形公式结果为    {}, Simposon公式结果为  {}, 柯特斯公式的结果为  {}",fn_tx(0.0, 1.0, f),fn_sim(0.0, 1.0, f), fn_cotes(0.0, 1.0, f));
    let e = std::f64::consts::E - 1.0;
    println!("e 取值{}  ",std::f64::consts::E);
    println!("(1):梯形公式误差为    {}, Simposon公式误差为  {}, 柯特斯公式的误差为  {}",fn_tx(0.0, 1.0, f) - e,fn_sim(0.0, 1.0, f) - e, fn_cotes(0.0, 1.0, f) - e);
}
