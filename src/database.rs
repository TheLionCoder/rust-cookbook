use std::collections::HashMap;
use rusqlite::{Connection, Result, Statement, Transaction};

#[derive(Debug)]
#[allow(dead_code)]
struct ZFighter {
    name: String,
    planet: String
}

pub fn create_db() -> Result<()> {
    let conn = Connection::open("assets/data/data.db")?;

    conn.execute(
        "create table if not exists z_fighters_planet (\
             id integer primary key,\
             name text not null unique)",
        (),
        )?;
    conn.execute(
        "create table  if not exists z_fighters (\
             id integer primary key,\
             name text not null,\
             planet_id integer not null references z_fighters_planet(id)\
             )",
        (),
    )?;
    Ok(())
}

pub fn insert_data() -> Result<()> {
   let conn: Connection = Connection::open("assets/data/data.db")?;

    let mut z_fighters_planet: HashMap<String, Vec<&str>> = HashMap::new();
    z_fighters_planet.insert(String::from("Planet Vegeta"), vec!["Goku", "Vegeta", "Raditz"]);
    z_fighters_planet.insert(String::from("Earth"), vec!["Gohan", "Krillin", "Yamcha"]);
    z_fighters_planet.insert(String::from("Namek"), vec!["Piccolo", "Dende", "Kamisama"]);

    for (planet, z_fighters) in &z_fighters_planet {
        conn.execute(
            "INSERT INTO z_fighters_planet (name) values (?1)",
            &[&planet.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for fighter in z_fighters {
            conn.execute(
                "INSERT INTO z_fighters (name, planet_id) values (?1, ?2)",
                &[&fighter.to_string(), &last_id],
            )?;
        }
    }
    Ok(())
}

pub fn select_data() -> Result<()> {
    let conn: Connection = Connection::open("assets/data/data.db")?;

    let mut stmt: Statement = conn.prepare(
        "SELECT zf.name, zfp.name FROM z_fighters zf \
INNER JOIN z_fighters_planet zfp \
ON zfp.id = zf.planet_id"
    )?;

    let fighters = stmt.query_map((), |row| {
        Ok(ZFighter{
            name: row.get(0)?,
            planet: row.get(1)?,
        })
    })?;

    for fighter in fighters {
        println!("Found fighter {:?}", fighter);
    }

    Ok(())
}


pub fn execute_transaction() -> Result<()> {
    let mut conn: Connection = Connection::open("assets/data/data.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    Ok(())

}

// auxiliar functions
fn successful_tx(conn: &mut Connection) -> Result<()> {
    let tx: Transaction = conn.transaction()?;

    tx.execute("delete from z_fighters_planet", ())?;
    tx.execute("insert into z_fighters_planet (name) values (?1)", &[&"Kaio-sama"])?;
    tx.execute("insert into z_fighters_planet (name) values (?1)", &["Namek"])?;

    tx.commit()
}

fn rolled_back_tx(conn: &mut Connection) -> Result<()> {
    let tx: Transaction = conn.transaction()?;

    tx.execute("delete from z_fighters_planet", ())?;
    tx.execute("insert into z_fighters_planet (name) values (?1)", &[&"Kaio-sama"])?;
    tx.execute("insert into z_fighters_planet (name) values (?1)", &[&"Namek"])?;
    tx.execute("insert into z_fighters_planet (name) values (?1)", &[&"Earth"])?;

    tx.execute("insert into z_fighters_planet (name) values (?1)", &[&"Earth"])?;

    tx.commit()
}