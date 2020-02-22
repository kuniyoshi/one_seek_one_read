fn main( ) {
    let count = 10;
    let iter = ( 1..count ).cycle( ).take( 42 );

    for v in iter {
        println!( "{}", v );
    }
}
