// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

use crate::cmp::{max, min};
use crate::collections::VecDeque;
use crate::io;
use crate::io::*;

use sgx_test_utils::test_case;

#[test_case]
fn copy_copies() {
    let mut r = repeat(0).take(4);
    let mut w = sink();
    assert_eq!(copy(&mut r, &mut w).unwrap(), 4);

    let mut r = repeat(0).take(1 << 17);
    assert_eq!(copy(&mut r as &mut dyn Read, &mut w as &mut dyn Write).unwrap(), 1 << 17);
}

struct ShortReader {
    cap: usize,
    read_size: usize,
    observed_buffer: usize,
}

impl Read for ShortReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = min(self.cap, self.read_size).min(buf.len());
        self.cap -= bytes;
        self.observed_buffer = max(self.observed_buffer, buf.len());
        Ok(bytes)
    }
}

struct WriteObserver {
    observed_buffer: usize,
}

impl Write for WriteObserver {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.observed_buffer = max(self.observed_buffer, buf.len());
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[test_case]
fn copy_specializes_bufwriter() {
    let cap = 117 * 1024;
    let buf_sz = 16 * 1024;
    let mut r = ShortReader { cap, observed_buffer: 0, read_size: 1337 };
    let mut w = BufWriter::with_capacity(buf_sz, WriteObserver { observed_buffer: 0 });
    assert_eq!(
        copy(&mut r, &mut w).unwrap(),
        cap as u64,
        "expected the whole capacity to be copied"
    );
    assert_eq!(r.observed_buffer, buf_sz, "expected a large buffer to be provided to the reader");
    assert!(w.get_mut().observed_buffer > DEFAULT_BUF_SIZE, "expected coalesced writes");
}

#[test_case]
fn copy_specializes_bufreader() {
    let mut source = vec![0; 768 * 1024];
    source[1] = 42;
    let mut buffered = BufReader::with_capacity(256 * 1024, Cursor::new(&mut source));

    let mut sink = Vec::new();
    assert_eq!(crate::io::copy(&mut buffered, &mut sink).unwrap(), source.len() as u64);
    assert_eq!(source.as_slice(), sink.as_slice());

    let buf_sz = 71 * 1024;
    assert!(buf_sz > DEFAULT_BUF_SIZE, "test precondition");

    let mut buffered = BufReader::with_capacity(buf_sz, Cursor::new(&mut source));
    let mut sink = WriteObserver { observed_buffer: 0 };
    assert_eq!(crate::io::copy(&mut buffered, &mut sink).unwrap(), source.len() as u64);
    assert_eq!(
        sink.observed_buffer, buf_sz,
        "expected a large buffer to be provided to the writer"
    );
}

#[test_case]
fn copy_specializes_to_vec() {
    let cap = 123456;
    let mut source = ShortReader { cap, observed_buffer: 0, read_size: 1337 };
    let mut sink = Vec::new();
    assert_eq!(cap as u64, io::copy(&mut source, &mut sink).unwrap());
    assert!(
        source.observed_buffer > DEFAULT_BUF_SIZE,
        "expected a large buffer to be provided to the reader"
    );
}

#[test_case]
fn copy_specializes_from_vecdeque() {
    let mut source = VecDeque::with_capacity(100 * 1024);
    for _ in 0..20 * 1024 {
        source.push_front(0);
    }
    for _ in 0..20 * 1024 {
        source.push_back(0);
    }
    let mut sink = WriteObserver { observed_buffer: 0 };
    assert_eq!(40 * 1024u64, io::copy(&mut source, &mut sink).unwrap());
    assert_eq!(20 * 1024, sink.observed_buffer);
}

#[test_case]
fn copy_specializes_from_slice() {
    let mut source = [1; 60 * 1024].as_slice();
    let mut sink = WriteObserver { observed_buffer: 0 };
    assert_eq!(60 * 1024u64, io::copy(&mut source, &mut sink).unwrap());
    assert_eq!(60 * 1024, sink.observed_buffer);
}

mod io_benches {
    use crate::fs::File;
    use crate::fs::OpenOptions;
    use crate::io::prelude::*;
    use crate::io::BufReader;

    use sgx_test_utils::bench_case;
    use sgx_test_utils::Bencher;

    #[bench_case]
    fn bench_copy_buf_reader(b: &mut Bencher) {
        let mut file_in = File::open("/dev/zero").expect("opening /dev/zero failed");
        // use dyn to avoid specializations unrelated to readbuf
        let dyn_in = &mut file_in as &mut dyn Read;
        let mut reader = BufReader::with_capacity(256 * 1024, dyn_in.take(0));
        let mut writer =
            OpenOptions::new().write(true).open("/dev/null").expect("opening /dev/null failed");

        const BYTES: u64 = 1024 * 1024;

        b.bytes = BYTES;

        b.iter(|| {
            reader.get_mut().set_limit(BYTES);
            crate::io::copy(&mut reader, &mut writer).unwrap()
        });
    }
}
