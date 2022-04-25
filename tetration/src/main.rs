fn main() {
    for i in 1..1645 {
        println!("{}", tetl((i as f64)/500.0, (i as f64)/500.0));
    }
}

fn tetl(a: f64, x: f64) -> f64 {
    if  x > 0.0 {
        let i = tetl(a, x-1.0);
        return a.powf(i);
    }
    else if x <= 0.0 || x > -1.0 {

        return 1.0 + x;
    }
    else {
        return 9999999999999.9;
    }
}
