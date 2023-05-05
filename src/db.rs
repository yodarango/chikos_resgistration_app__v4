use mysql_async::{Pool, Conn, Error, Opts};
use std::convert::Infallible;
use warp::{Filter};

pub async fn get_connection () -> Result<Pool, Error> {
    let db_url = Opts::from_url("mysql://root:root@localhost/scholar_dev?socket=%2Ftmp%2Fmysql.sock")?;
    let pool = Pool::new(db_url);
    // pass the connection and the pool to the caller
    Ok(pool)
}

// middleware to pass the connection pool to the handler
pub fn with_db(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}


pub mod queries {
    pub mod get {
        use mysql_async::{prelude::*, Pool, Conn, Error, Opts};
        use crate::models::Registrant;

        pub async fn get_all_users(pool: Pool) -> Result<Vec<Registrant>, Error> {
        

                let mut conn = match pool.get_conn().await {
                    Ok(conn) => conn,
                    Err(e) => panic!("couldn't get connection: {}", e),
                };
                
                let users = "SELECT ID, signature FROM users"
                .with(())
                .map(&mut conn, |(ID, signature)| Registrant { id: ID, name: signature })
                .await?;
                
                print!("{:?}", users);

                drop(conn);

                pool.disconnect().await?;

                Ok(users)
        }
    }
}