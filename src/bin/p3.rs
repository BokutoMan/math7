fn f1(x:f64)->f64
{
    let mut re = 4.0f64;
    re /= x*x + 1.0;
    re
}
fn f2(x:f64)->f64
{
    let re = 1.0/x;
    re
}

// Romberg算法
fn fn_rom<F>(ep:f64, a: f64, b:f64, f:F)->f64
    where F:Fn(f64)->f64,
{
    // (1)
    let mut k = 0;
    let mut h = b-a;
    let t1 = h/2.0 * (f(a) + f(b));
    println!("{}",t1);
    // (2)
    h = h/2.0;
    k = k+1;
    let t2 = t1/2.0 + h * f(a + h);
    let s1 = 4.0/3.0 * t2 - 1.0/3.0 * t1;
    println!("{}    {}",t2,s1);
    // (3)
    h = h/2.0;
    k = k+1;
    let t4 = t2/2.0 + h* (f(a+h) + f(a + 3.0*h));
    let s2 = 4.0/3.0 * t4 - 1.0/3.0 * t2;
    let c1 = 16.0/15.0 * s2 - 1.0/15.0 * s1;
    println!("{}    {}      {}",t4,s2,c1);
    // (4)
    h = h/2.0;
    k = k+1;
    let mut m = 0.0f64;
    for i in 1..5{
        m += f(a + (2*i - 1) as f64 *h);
    }
    let t8 = 0.5 * t4 + h * m;
    let s4 = 4.0/3.0 * t8 - 1.0/3.0 * t4;
    let c2 = 16.0/15.0 * s4 - 1.0/15.0 * s1;
    let r1 = 64.0/63.0 * c2 - 1.0/63.0 * c1;
    println!("{}    {}      {}      {}",t8,s4,c2,r1);
    // (5)
    let mut t_2k_1 = t8;
    let mut s_2k_2 = s4;
    let mut c_2k_3 = c2;
    let mut r_2k_4 = r1;
    loop{
        h = h/2.0;
        k = k + 1;
        let mut m = 0.0f64;
        let max = 2i32.pow(k-1) + 1;
        for i in 1..max{
            m += f(a + (2*i - 1) as f64 *h);
        }
        let t_2k = 0.5 * t_2k_1 + h * m;
        let s_2k_1 = 4.0/3.0 * t_2k - 1.0/3.0 * t_2k_1;
        let c_2k_2 = 16.0/15.0 * s_2k_1 - 1.0/15.0 * s_2k_2;
        let r_2k_3 = 64.0/63.0 * c_2k_2 - 1.0/63.0 * c_2k_3;
        println!("{}    {}      {}      {}",t_2k, s_2k_1,c_2k_2,r_2k_3);
        if (r_2k_3 - r_2k_4).abs() < ep {
            r_2k_4 = r_2k_3;
            break;
            }
        else {
            t_2k_1 = t_2k;
            s_2k_2 = s_2k_1;
            c_2k_3 = c_2k_2;
            r_2k_4 = r_2k_3;
        }
    }
    r_2k_4
}

fn main() {
    println!("(1):计算中间结果如下：");
    println!("(1):龙贝格公式计算结果为：{}", fn_rom(0.5e-5, 0.0, 1.0, f1));
    println!("(2):计算中间结果如下：");
    println!("(2):龙贝格公式计算结果为：{}", fn_rom(0.5e-5, 1.0, 3.0, f2));
}
