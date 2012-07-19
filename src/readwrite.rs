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