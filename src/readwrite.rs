use std;

import io::file_writer;
import io::file_reader;
import io::reader;
import io::writer;
import result::result;
import result::is_err;
import result::get_err;
import result::unwrap;
import io::reader_util;
import io::writer_util;

fn is_success (-path: str) {
    let maybe_test_reader: result<reader, str> = file_reader(path);

    if is_err::<reader, str>(maybe_test_reader) {
        #warn("%s", get_err(maybe_test_reader));
        assert false;
    }

    let test_reader: reader = unwrap(maybe_test_reader);
    
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

fn freader (-path: str) -> reader {
    let maybe_reader = file_reader(path);

    if is_err::<reader, str>(maybe_reader) {
        #warn("%s", get_err(maybe_reader));
        assert false;
    }

    unwrap(maybe_reader)
}

fn fwriter (-path: str) -> writer {
    let maybe_writer = file_writer(path, ~[io::append]);

    if is_err::<writer, str>(maybe_writer) {
        #warn("%s", get_err(maybe_writer));
        assert false;
    }

    unwrap(maybe_writer)
}

fn is_empty (-path: str) -> bool {
    freader(path).read_byte() == -1
}

fn is_success2 (-path: str) {
    let reader = freader(path);
    assert reader.read_line() == "success";
    assert reader.eof();
}

fn clear_file (-path: str) {
    let reader = freader(path);
    /* How do I do this? */
    log(debug, ~"Unimplemented");
}

#[test]
fn read_absolute_file () {
    is_success("/home/havvy/read.txt");
}

#[test]
fn read_relative_file () {
    is_success("./read.txt");
    is_success("read.txt");
}

#[test]
fn utility_read_fns () {
    is_success2("read.txt");
    assert freader("read.txt").read_whole_stream() == 
        ~[115, 117, 99, 99, 101, 115, 115];
    
    freader("read.txt").each_line(fn@ (line: str) -> bool {
        assert line == "success"; true
    });
}

#[test]
fn write_dot_txt_is_empty () {
    assert is_empty("write.txt");
}

#[test]
fn write_to_file () {
    assert is_empty("write.txt");
    let test_writer = fwriter("write.txt");
    test_writer.write_str("success");
    is_success2("write.txt");
    clear_file("write.txt");
}