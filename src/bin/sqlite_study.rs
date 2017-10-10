extern crate rusqlite;
extern crate time;

use time::Timespec;

use rusqlite::{Connection, DatabaseName, OpenFlags};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    time_created: Timespec,
    data: Option<Vec<u8>>,
}

use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;

fn main() {
    if Path::new("data.db").exists() {
        {
            let rc = Connection::open("file:data.db?mode=ro").expect("");

            let now = Instant::now();

            let mut stmt = rc.prepare("SELECT id, name, time_created, data FROM person").unwrap();

            let person_iter = stmt.query_map(&[], |row| {
                Person {
                    id: row.get(0),
                    name: row.get(1),
                    time_created: row.get(2),
                    data: row.get(3)
                }
            }).unwrap();

            let count = person_iter.count();

            println!("read data.db cost:{:?},count:{}", Instant::now().duration_since(now), count);
        }
        fs::remove_file("data.db").expect("");
    }
    if Path::new("data_bk.db").exists() {
        {
            let rc = Connection::open("file:data_bk.db?mode=ro").expect("");

            let now = Instant::now();

            let mut stmt = rc.prepare("SELECT id, name, time_created, data FROM person").unwrap();

            let person_iter = stmt.query_map(&[], |row| {
                Person {
                    id: row.get(0),
                    name: row.get(1),
                    time_created: row.get(2),
                    data: row.get(3)
                }
            }).unwrap();

            let count = person_iter.count();

            println!("read data_bk.db cost:{:?},count:{}", Instant::now().duration_since(now), count);
        }
        fs::remove_file("data_bk.db").expect("");
    }

    let mut wc = Connection::open("file:data.db?mode=rwc").expect("");

    wc.query_row("PRAGMA journal_mode;", &[], |row| {
        let t: String = row.get(0);
        println!("journal_mode={}", t);
    }).expect("");

    wc.query_row("PRAGMA journal_mode;", &[], |row| {
        let t: String = row.get(0);
        println!("journal_mode={}", t);
    }).expect("");

    wc.execute("CREATE TABLE person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )", &[]).expect("");
    use std::sync::Arc;

    use std::sync::atomic::{Ordering, ATOMIC_BOOL_INIT};

    let flag = Arc::new(ATOMIC_BOOL_INIT);

    for _ in 0..15 {
        let inner = Arc::clone(&flag);

        std::thread::spawn(move || {
            let rc = Connection::open("file:data.db?mode=rw").expect("");

            while !inner.load(Ordering::Relaxed) {
                let now = Instant::now();

                let mut stmt = rc.prepare("SELECT id, name, time_created, data FROM person").unwrap();

                let person_iter = stmt.query_map(&[], |row| {
                    Person {
                        id: row.get(0),
                        name: row.get(1),
                        time_created: row.get(2),
                        data: row.get(3)
                    }
                }).unwrap();

                let count = person_iter.count();

                println!("read cost:{:?},count:{}", Instant::now().duration_since(now), count);

                std::thread::sleep(Duration::from_millis(500));
            }
        });
    }

    let do_backup = Arc::new(ATOMIC_BOOL_INIT);

    let inner2 = Arc::clone(&do_backup);

    std::thread::spawn(move || {
        while !inner2.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(500));
        }

        let now = Instant::now();

        let src = Connection::open("file:data.db?mode=rw").expect("");

        src.backup(DatabaseName::Main, "data_bk.db", Some(|progress| {
            println!("backup:{:?}", progress);
        })).expect("");
        println!("backup cost:{:?}", Instant::now().duration_since(now));
    });

    println!("begin write");
    let now = Instant::now();

    for i in 0..100 {
        let tx = wc.transaction().expect("");

        if i == 90 {
            do_backup.store(true, Ordering::Relaxed);
        }

        {
            let mut insert = tx.prepare("INSERT INTO person (name, time_created, data) VALUES (?1, ?2, ?3)").expect("");

            for j in 0..10_000 {
                let me = Person {
                    id: i * 10_000 + j,
                    name: format!("Steven_{}", i * 10_000 + j),
                    time_created: time::get_time(),
                    data: None
                };

                insert.insert(&[&me.name, &me.time_created, &me.data]).expect("");
            }
        }

        tx.execute_batch("PRAGMA wal_checkpoint(FULL);").expect("");

        tx.commit().expect("");
    }

    println!("write cost:{:?}", Instant::now().duration_since(now));


    flag.store(true, Ordering::Relaxed);

    {
        let rc = Connection::open("file:data.db?mode=ro&cache=shared").expect("");

        let now = Instant::now();

        let mut stmt = rc.prepare("SELECT id, name, time_created, data FROM person").unwrap();

        let person_iter = stmt.query_map(&[], |row| {
            Person {
                id: row.get(0),
                name: row.get(1),
                time_created: row.get(2),
                data: row.get(3)
            }
        }).unwrap();

        let count = person_iter.count();

        println!("read cost:{:?},count:{}", Instant::now().duration_since(now), count);
    }


}