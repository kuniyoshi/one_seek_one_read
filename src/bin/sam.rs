fn main( ) {
    let count = 10;
    let iter = ( 1..count ).cycle( ).take( 42 );

    for v in iter {
        println!( "{}", v );
    }

    call( ( 1 .. 10 ).cycle( ).take( 40 ) );
}

fn call<U>( a: U )
    where U: IntoIterator<Item=i32>
{ }
