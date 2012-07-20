Writing and reading files. Important in any language. Here's how to do so in
Rust. This page will focus on the API and not on the language constructs.

As a warning, I'm not at all comfortable with Rust yet. There are parts of the
syntax that completely confuse me, including the majority of pointers. I've
never written a line of C++ and only once used pointers in C. Managing memory
is the reason I even chose to learn this language.

And finally, I'm learning this all through experimentation and questions in
IRC. Take the information in here with a grain of salt, and if there is an
error, be bold and send a pull request or file an issue. As I become more sure
of what I'm saying, I'll drop the pronouns.

## An Initial Look

To start out, we look at the core::io library(1). Just by looking at the
docpage, we make note of the following functions:

* FILE_reader
* FILE_writer
* file_writer
* file_reader
* mk_file_writer

The FILE variants read C Style files while the file variants read Rust style
files. I am not sure what the difference is between mk_file_writer and
file_writer. mk_file_writer is just chain(file_writer). 

I'm going to see if I can get away with using the Rust style files, and not
deal with C style files. As such, the only two looked at will be file_reader
and file_writer.

Both take a path of type ~str. They return a result<reader/writer, ~str>. So,
to understand file io in Rust, you must also understand the result type.

## The result type

The result type appears to be Rust's solution to runtime errors. A result is
an enum of either ok(T) or err(U). Results must be unpacked before they are
used, but for the programs on this page, I will first assert that they are ok.

The following three functions will be used from core::result(2).

1. is_err<T, E>(result<T, E>) -> bool: True iff result<T, E> is err(E).
2. unpack<T, E>(result<T, E>) -> T: Return T in ok(T) if result is ok(T).
Otherwise, fails.

More functions, including methods for chaining and iterating can be found in
the module. They will not be used here.

## Testing

The code shown here are all tests, or helper functions for tests. As such,
assertions will test for expected results of running the test. The tests are
also written expecting that the program is at /home/havvy and that there is a
file test.txt containing the contents "success" in the same directory.

The io and result functions, traits, and types are imported in the header.

### Reading Files

To begin, let us read bytes from test.txt.

## The Testing Function

~~~~
fn is_success (-path: str) {
    // [1]
    let maybe_test_reader: result<reader, str> = file_reader(path);

    // [2]
    if is_err::<reader, str>(maybe_test_reader) {
        io::println(result::get_err(maybe_test_reader));
        assert false;
    }
    let test_reader: reader = result::unwrap(maybe_test_reader);
    
    // [3] [4]
    let mut bytes: ~[u8] = ~[];
    loop {
        let byte: int = test_reader.read_byte();
        #debug("%d", byte);
        if test_reader.eof() { break }
        vec::push(bytes, byte as u8);
    }

    // [5]
    assert bytes == ~[115, 117, 99, 99, 101, 115, 115];
    let maybe_success: str = str::from_bytes(bytes);
    assert maybe_success == "success";
}
~~~~

1. The function tries to creates a file_reader using the path given to it.
2. If the file_reader call failed, print why and then fail the test.
Otherwise, unwrap the result.
3. Since I do not yet see a way of querying the length of a file, I will just
read the file one byte at a time, and store it in a vector. The bytes are read
in as an int, but str::from_bytes expects a uint. So each byte is casted to
a uint. This happens continously until 'end of file' is reached.
4. The reader.eof() method returns true when the currently read
byte is eof. If this is the case, then byte will be -1 (and thus the reason
reader.read_byte returns an int instead of a u8) which should not be added to
the vector. Since this is the EOF, the code breaks from the loop.
5. At this point, the assertions that the file contains the correct contents
is checked twice. Once in byte form, and then again as a string. The byte form
is the ASCII values (and thus, UTF8 values) for 'success'.

### Why not a while loop?

You might be asking, why am I checking to exit the loop from inside the loop
and not having a check between iterations.

This is due to the way reader.eof() works. Using a while loop checking for eof 
will cause the EOF byte of value -1 (255u8) to be appended to the vector. The
final value has to be popped off when using a while loop.

~~~~
let mut bytes: ~[u8] = ~[];
while !reader.eof() {
    vec::push(bytes, reader.read_byte() as u8);
}
vec::pop(bytes);
~~~~

### Absolute Path

This test will work when the program is ran from anywhere as long as test.txt
exists in the specified location. 

An absolute URL begins with the root directory, '/'.

~~~~
#[test]
fn read_absolute_file () {
    is_success("/home/havvy/test.txt");
}
~~~~

### Relative Path

A relative path is any path that does not begin with the root directory. The
path starts from the directory the program is called from. For example, if the
program is called from /home/havvy/, then a path of test.txt will read from
/home/havvy/test.txt.

Directories named '.' and '..' has special meaning. '.' will go to the current
directory, while '..' goes one directory up.

The paths test.txt and ./test.txt are equivelent. The following two tests will
either both pass or both fail, depending on whether or not the file exists in
the directory the program is ran from and whether or not the file contains the
contents "success".

~~~~
#[test]
fn read_relative_file () {
    is_success("./test.txt");
}

#[test]
fn read_relative_file_2 () {
    is_success("test.txt");
}
~~~~

## Writing Files



## References

1. http://dl.rust-lang.org/doc/0.3/core/io.html
2. http://dl.rust-lang.org/doc/0.3/core/result.html