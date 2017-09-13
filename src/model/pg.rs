use postgres::{Connection, TlsMode};

pub fn get_conn() -> Connection {
    
       Connection::connect("postgres://rforum:rforum@localhost:5432/rforum_cn", TlsMode::None).unwrap()

}
