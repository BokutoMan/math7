fn fn3(x:f64)->f64
{
    let x0 = 1.0;
    let x1 = 1.1;
    let x2 = 1.2;
    let f0 = 0.25;
    let f1 = 0.226757;
    let f2 = 0.206612;
    let mut re = f0 * ((x - x1 + x - x2)/((x0 - x1)*(x0 - x2)));
    re += f1 * ((x - x0 + x - x2)/((x1 - x0)*(x1 - x2)));
    re += f2 * ((x - x0 + x - x1)/((x2 - x0)*(x2 - x1)));
    re
}

fn main() {
    println!("f(x)在1.0处的倒数近似值为：{:.5}",fn3(1.0));
    println!("f(x)在1.1处的倒数近似值为：{:.5}",fn3(1.1));
    println!("f(x)在1.2处的倒数近似值为：{:.5}",fn3(1.2));
}
