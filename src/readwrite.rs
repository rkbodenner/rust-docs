use std;

import io::file_writer;
import io::file_reader;
import io::reader;
import io::writer;
import result::result;
import result::is_err;
import result::unwrap;

fn is_success (-path: str) {
    let maybe_test_reader: result<reader, str> = file_reader(path);

    if is_err::<reader, str>(maybe_test_reader) {
        #info("%s", result::get_err(maybe_test_reader));
        assert false;
    }

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

#[test]
fn read_absolute_file () {
    is_success("/home/havvy/test.txt");
}

#[test]
fn read_relative_file () {
    is_success("./test.txt");
}

#[test]
fn read_relative_file_2 () {
    is_success("test.txt");
}