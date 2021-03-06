// Copyright 2014 Tyler Neely
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use exonum_rocksdb::DB;
use tempdir::TempDir;

#[test]
pub fn test_forwards_iteration() {
    let temp_dir = TempDir::new("forwards_iteration").unwrap();
    let db = DB::open_default(temp_dir.path()).unwrap();
    db.put(b"k1", b"v1").unwrap();
    db.put(b"k2", b"v2").unwrap();
    db.put(b"k3", b"v3").unwrap();
    db.put(b"k4", b"v4").unwrap();

    let mut iter = db.raw_iterator();
    iter.seek_to_first();

    assert_eq!(iter.valid(), true);
    assert_eq!(iter.key(), Some(b"k1".to_vec()));
    assert_eq!(iter.value(), Some(b"v1".to_vec()));

    iter.next();

    assert_eq!(iter.valid(), true);
    assert_eq!(iter.key(), Some(b"k2".to_vec()));
    assert_eq!(iter.value(), Some(b"v2".to_vec()));

    iter.next(); // k3
    iter.next(); // k4
    iter.next(); // invalid!

    assert_eq!(iter.valid(), false);
    assert_eq!(iter.key(), None);
    assert_eq!(iter.value(), None);
}

#[test]
pub fn test_seek_last() {
    let temp_dir = TempDir::new("backwards_iteration").unwrap();
    let db = DB::open_default(temp_dir.path()).unwrap();
    db.put(b"k1", b"v1").unwrap();
    db.put(b"k2", b"v2").unwrap();
    db.put(b"k3", b"v3").unwrap();
    db.put(b"k4", b"v4").unwrap();

    let mut iter = db.raw_iterator();
    iter.seek_to_last();

    assert_eq!(iter.valid(), true);
    assert_eq!(iter.key(), Some(b"k4".to_vec()));
    assert_eq!(iter.value(), Some(b"v4".to_vec()));

    iter.prev();

    assert_eq!(iter.valid(), true);
    assert_eq!(iter.key(), Some(b"k3".to_vec()));
    assert_eq!(iter.value(), Some(b"v3".to_vec()));

    iter.prev(); // k2
    iter.prev(); // k1
    iter.prev(); // invalid!

    assert_eq!(iter.valid(), false);
    assert_eq!(iter.key(), None);
    assert_eq!(iter.value(), None);
}

#[test]
pub fn test_seek() {
    let temp_dir = TempDir::new("seek").unwrap();
    let db = DB::open_default(temp_dir.path()).unwrap();
    db.put(b"k1", b"v1").unwrap();
    db.put(b"k2", b"v2").unwrap();
    db.put(b"k3", b"v3").unwrap();
    db.put(b"k4", b"v4").unwrap();

    let mut iter = db.raw_iterator();
    iter.seek(b"k2");

    assert_eq!(iter.valid(), true);
    assert_eq!(iter.key(), Some(b"k2".to_vec()));
    assert_eq!(iter.value(), Some(b"v2".to_vec()));
}

#[test]
pub fn test_seek_to_nonexistant() {
    let temp_dir = TempDir::new("seek_to_nonexistant").unwrap();
    let db = DB::open_default(temp_dir.path()).unwrap();
    db.put(b"k1", b"v1").unwrap();
    db.put(b"k3", b"v3").unwrap();
    db.put(b"k4", b"v4").unwrap();

    let mut iter = db.raw_iterator();
    iter.seek(b"k2");

    assert_eq!(iter.valid(), true);
    assert_eq!(iter.key(), Some(b"k3".to_vec()));
    assert_eq!(iter.value(), Some(b"v3".to_vec()));
}
