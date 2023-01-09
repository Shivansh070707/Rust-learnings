fn main() {
    let mut car: String = String::from("shivansh");
    display2(&mut car);
    println!("{car}")
}
fn display2(st: &mut String) {
    st.push_str(" Hello !");
    println!("{st}");
}
