extern crate msql_srv;
extern crate mysql;

use msql_srv::*;
use std::io;
use std::net;
use std::thread;
use nom_sql::parser::parse_query;

struct Backend;
// impl<W: io::Write> MysqlShim<W> for Backend {
//     type Error = io::Error;

//     fn on_prepare(&mut self, _: &str, info: StatementMetaWriter<W>) -> io::Result<()> {
//         info.reply(42, &[], &[])
//     }
//     fn on_execute(
//         &mut self,
//         _: u32,
//         _: msql_srv::ParamParser,
//         results: QueryResultWriter<W>,
//     ) -> io::Result<()> {
//         results.completed(0, 0)
//     }
//     fn on_close(&mut self, _: u32) {}

//     fn on_query(&mut self, _: &str, results: QueryResultWriter<W>) -> io::Result<()> {
//         results.start(&[])?.finish()
//     }
// }

struct User {
    id: i32,
    name: String,
}

impl<W: io::Write> MysqlShim<W> for Backend {
    type Error = io::Error;

    fn on_prepare(&mut self, _: &str, info: StatementMetaWriter<W>) -> io::Result<()> {
        info.reply(42, &[], &[])
    }
    fn on_execute(
        &mut self,
        _: u32,
        _: ParamParser,
        results: QueryResultWriter<W>,
    ) -> io::Result<()> {
        results.completed(0, 0)
    }
    fn on_close(&mut self, _: u32) {}

    fn on_init(&mut self, _: &str, _writer: InitWriter<W>) -> io::Result<()> { Ok(()) }

    fn on_query(&mut self, query: &str, results: QueryResultWriter<W>) -> io::Result<()> {
        let users = vec![
            User{id: 1, name: "Huseyin".to_string()},
            User{id: 2, name: "Ayse".to_string()},
            User{id: 3, name: "Sila".to_string()},
            User{id: 4, name: "Mert".to_string()},
            User{id: 5, name: query.to_string()},
        ];
        let parsed_query = parse_query(query);
        println!("{:?}", parsed_query);
        let cols = [
            Column {
                table: "user".to_string(),
                column: "id".to_string(),
                coltype: ColumnType::MYSQL_TYPE_LONGLONG,
                colflags: ColumnFlags::empty(),
            },
            Column {
                table: "user".to_string(),
                column: "name".to_string(),
                coltype: ColumnType::MYSQL_TYPE_STRING,
                colflags: ColumnFlags::empty(),
            },
        ];
        let mut rw = results.start(&cols)?;
        for u in users {
            rw.write_col(u.id)?;
            rw.write_col(u.name)?;
            rw.end_row()?;
        }
        rw.finish()
    }
}

fn main() {
    let mut threads = Vec::new();
    let listener = net::TcpListener::bind("127.0.0.1:3306").unwrap();

    while let Ok((s, _)) = listener.accept() {
        threads.push(thread::spawn(move || {
            MysqlIntermediary::run_on_tcp(Backend, s).unwrap();
        }));
    }

    for t in threads {
        t.join().unwrap();
    }
}
