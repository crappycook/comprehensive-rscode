#[allow(unused)]
fn dive_info_sharedref() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
}

#[allow(unused)]
// “Exclusive” means that only this reference can be used to access the value.
// No other references (shared or exclusive) can exist at the same time,
// and the referenced value cannot be accessed while the exclusive reference exists.
pub fn dive_into_exclusive_ref() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    let y_coord = &mut point.1;
    // let x2_coord = &point.0; // not ok
    *x_coord = 20;
    point.0 = 100; // ok
    *y_coord = 30;
    println!("point: {point:?}");
}
