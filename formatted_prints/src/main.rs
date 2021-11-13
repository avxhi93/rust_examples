fn main() {
    // In general the {} will be automatically replaced with any arguments
    // these will be stringified
    println!("{} days", 31);


    // without a suffix 31 becomes i32

    // optional patterns this work with positional arguments
    println!("{0}, this is {1}. {1} this is {0}", "Alice", "Bob");

    // as can named arguments
    println!("{subject}{verb}{object}", 
        object="the lazy dog",
        verb="jumps over",
        subject="the quick brown fox");

    // special formatting can be specified after a : 
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);


    // you can right-align text with specified width. This will output 1. 5 white spaces and a 1
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0<width$}", number = 1, width=6);
    println!("{number:0>width$}", number = 1, width=6);

    // rust even checks to make the correct number of arguments are used
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // create a structure named 'Structure'which contains an i32
    #[derive(Debug)]
    struct Structure(i32);

    println!("This struct {:?} wont print... ", Structure(3));
}
