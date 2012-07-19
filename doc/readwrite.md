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

The following two functions will be used from core::result(2).

1. is_ok<T, E>(result<T, E>) -> bool: True iff result<T, E> is ok(T).
2. unpack<T, E>(result<T, E>) -> T: Return T in ok(T) if result is ok(T).
Otherwise, fails.

More functions, including methods for chaining and iterating can be found in
the module. They will not be used here.

## An Initial Program

~~~~
use std;

import io::file_writer;
import io::file_reader;
import io::reader;
import io::writer;
import result::result;

#[test]
#[doc="I created a file at /home/havvy/test.txt with the following contents:

'success'
"]
fn read_absolute_file () {
    let path: str = "/home/havvy/test.txt";
    let maybe_test_reader: result<reader, str> = file_reader(path);

    assert result::is_ok::<reader, str>(maybe_test_reader);
    let test_reader: reader = result::unwrap(maybe_test_reader);
    
    let mut bytes: ~[u8] = ~[];
    loop {
        let byte: int = test_reader.read_byte();
        #debug("%d", byte);
        if test_reader.eof() { break }
        vec::push(bytes, byte as u8);
    }

    assert bytes == ~[115, 117, 99, 99, 101, 115, 115];
    let maybe_success: str = str::from_bytes(bytes);
    assert maybe_success == "success";
}
~~~~

Sorry for all the imports. I wrote this program as a test, and got it to pass.
Here's the process used for reading the file:

1. Create a file_reader passing it an absolute path. The path leads to a file
I put in my home directory called test.txt with the contents 'success' encoded
in ascii.
2. file_reader doesn't give us a reader right away. I assert that the result
is okay, and if it isn't, let the program die. I could have just left this to
unwrap, but wanted to catch the error on my program line. The file_reader is
then unwrapped.
3. Since I do not yet see a way of querying the length of a file, I will just
read the file one byte at a time, and store it in a vector. The bytes are read
in as an int, but str::from_bytes expects a uint. So the bytes are casted to
their proper type. This happens continously until 'end of file' is reached.
4. The reader.eof() method returns true when the currently read
byte is eof. If this is the case, then byte will be -1 (and thus the reason
reader.read_byte returns an int instead of a u8) and should not be added to
the vector. Since there is no more content, the code breaks from the loop.
5. At this point, the entire file is read. The rest of the code is just making
sure that the file is read properly.

## References

1. http://dl.rust-lang.org/doc/0.3/core/io.html
2. http://dl.rust-lang.org/doc/0.3/core/result.html