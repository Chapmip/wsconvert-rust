mod asciify;

fn main() {
    let mut buf = [ 185; 32 ];
    println!("before conversion = {:?}", buf);
    println!("length = {}", buf.len());

    let res = asciify::convert(&mut buf);
    println!("after conversion = {:?}", res);
    println!("length = {}", res.len());
}
