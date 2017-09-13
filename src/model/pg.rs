use postgres::{Connection, TlsMode};

pub fn get_conn() -> Connection {
    
       Connection::connect("DATABASE_URL", TlsMode::None).unwrap()

}
