use rusqlite::{Connection, Result};
use crate::models::Connection as NetConnection;
use crate::api::ConnectionRow;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("netwatch.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS connections (
            id INTEGER PRIMARY KEY AUTOINCREMENT,

            timestamp INTEGER NOT NULL,

            src_ip TEXT NOT NULL,
            dst_ip TEXT NOT NULL,

            src_port INTEGER NOT NULL,
            dst_port INTEGER NOT NULL,

            old_state INTEGER NOT NULL,
            new_state INTEGER NOT NULL
        )
        ",
        [],
    )?;

    Ok(conn)
}

pub fn insert_connection(
    conn: &Connection,
    ev: &NetConnection,
) -> Result<()> {
    conn.execute(
        "
        INSERT INTO connections (
            timestamp,
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            old_state,
            new_state
        )
        VALUES (?, ?, ?, ?, ?, ?, ?)
        ",
        (
            ev.timestamp,
            &ev.src_ip,
            &ev.dst_ip,
            ev.src_port,
            ev.dst_port,
            ev.old_state,
            ev.new_state,
        ),
    )?;

    Ok(())
}

pub fn get_connections(
    conn: &Connection,
) -> Result<Vec<ConnectionRow>> {

    let mut stmt = conn.prepare(
        "
        SELECT
            id,
            timestamp,
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            old_state,
            new_state
        FROM connections
        ORDER BY id DESC
        LIMIT 100
        "
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(ConnectionRow {
            id: row.get(0)?,
            timestamp: row.get(1)?,

            src_ip: row.get(2)?,
            dst_ip: row.get(3)?,

            src_port: row.get(4)?,
            dst_port: row.get(5)?,

            old_state: row.get(6)?,
            new_state: row.get(7)?,
        })
    })?;

    let mut result = Vec::new();

    for row in rows {
        result.push(row?);
    }

    Ok(result)
}
