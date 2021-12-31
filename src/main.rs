use std::{env, thread, time};
use std::process::exit;

use postgres::{Client, Error, NoTls};

fn main() -> Result<(), Error> {
    if let Ok(connection_url) = env::var("DB_URL") {
        loop {
            check_tickets(&*connection_url);
            thread::sleep(time::Duration::from_secs(10));
        }
    } else {
        println!("DB_URL environment variable not found");
        exit(-1)
    }
}

fn check_tickets(connection_url: &str) -> Result<(), Error> {
    let mut client = Client::connect(connection_url, NoTls)?;

    // and t.status = 'ACQUIRED'
    let rows = client.query("select t.category_id, c.name, t.status, count(t.id), c.max_tickets from ticket t join ticket_category c on t.category_id = c.id
where t.event_id = (select id from event where short_name = '2022')
and c.name ~ '^(Blind|Early)'
and t.status <> 'INVALIDATED'
group by t.status, t.category_id, c.name, c.max_tickets
order by t.category_id", &[])?;

    if !rows.is_empty() {
        print_header();
        for row in rows {
            // let category_id: i32 = row.get(0);
            let category_name: &str = row.get(1);
            let status: &str = row.get(2);
            let count: i64 = row.get(3);
            let max_tickets: i32 = row.get(4);

            println!("{:<20} {:<12} {}/{}", category_name, status.to_lowercase(), count, max_tickets);
        }
        println!();
    }
    Ok(())
}

fn print_header() {
    println!("{:<19} {:<12} {}", "CATEGORY", "STATUS", "SOLD/MAX");
}
