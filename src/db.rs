use rusqlite::{
    Connection,
    Result,
};

use crate::api::ConnectionRow;
use crate::models::Connection as NetConnection;

pub fn init_db() -> Result<Connection> {

    let conn =
        Connection::open("netwatch.db")?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS connections (

            id INTEGER PRIMARY KEY AUTOINCREMENT,

            pid INTEGER NOT NULL,

            process_name TEXT,
            executable TEXT,

            timestamp INTEGER,

            src_ip TEXT NOT NULL,
            dst_ip TEXT NOT NULL,

            src_port INTEGER NOT NULL,
            dst_port INTEGER NOT NULL,

            old_state INTEGER NOT NULL,
            new_state INTEGER NOT NULL,

            risk_score INTEGER,
            threat_label TEXT
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
        INSERT INTO connections
        (
            pid,

            timestamp,

            src_ip,
            dst_ip,

            src_port,
            dst_port,

            old_state,
            new_state,
            hostname
        )
        VALUES
        (
            ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        ",
        (
            ev.pid,

            &ev.process_name,
            &ev.executable,

            ev.timestamp,

            &ev.src_ip,
            &ev.dst_ip,

            ev.src_port,
            ev.dst_port,

            ev.old_state,
            ev.new_state,

            &ev.hostname,
            &ev.risk_score,
            &ev.threat_label,
        ),
    )?;

    Ok(())
}

pub fn get_connections(
    conn: &Connection,
) -> Result<Vec<ConnectionRow>> {

    let mut stmt =
        conn.prepare(
            "
            SELECT
                id,
                pid,
                timestamp,
                src_ip,
                dst_ip,
                hostname,
                src_port,
                dst_port,
                old_state,
                new_state
            FROM connections
            ORDER BY id DESC
            LIMIT 500
            "
        )?;

    let rows = stmt.query_map([], |row| {

        Ok(ConnectionRow {

            id: row.get(0)?,

            pid: row.get(1)?,

            timestamp: row.get(2)?,

            src_ip: row.get(3)?,
            dst_ip: row.get(4)?,

            src_port: row.get(5)?,
            dst_port: row.get(6)?,

            old_state: row.get(7)?,
            new_state: row.get(8)?,

            hostname: row.get(9)?,

            process_name: row.get(10)?,
            executable: row.get(11)?,

            risk_score: row.get(12)?,
            threat_label: row.get(13)?,
        })

    })?;

    let mut result = Vec::new();

    for row in rows {
        result.push(row?);
    }

    Ok(result)
}
