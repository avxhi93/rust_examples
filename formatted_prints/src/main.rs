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
}
