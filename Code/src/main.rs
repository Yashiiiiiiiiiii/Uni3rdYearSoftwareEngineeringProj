
use mysql::*;
use mysql::prelude::*;



fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let url = "mysql://sql2102675:7377HBPLYpHE@lochnagar.abertay.ac.uk/sql2102675".trim();
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    /* 
    conn.query_drop(
        r"CREATE TEMPORARY TABLE payment (
            customer_id int not null,
            amount int not null,
            account_name text
        )"
    )?;

    */


    Ok(())
}