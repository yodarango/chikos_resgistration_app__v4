use mysql_async::{Pool, Conn, Error, Opts};
use std::convert::Infallible;
use warp::{Filter};

pub async fn get_connection () -> Result<Pool, Error> {
    let db_url = Opts::from_url("mysql://root:root@localhost/chikios?socket=%2Ftmp%2Fmysql.sock")?;
    let pool = Pool::new(db_url);
    Ok(pool)
}

// middleware to pass the connection pool to the handler
pub fn with_db(pool: Pool) -> impl Filter<Extract = (Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub mod queries {
    pub mod get {
        use mysql_async::{prelude::*, Pool, Error};
        use crate::models::Registrant;

        pub async fn get_all_users(from_id: String, pool: Pool) -> Result<Vec<Registrant>, Error> {
        
                let mut conn = match pool.get_conn().await {
                    Ok(conn) => conn,
                    Err(e) => panic!("couldn't get connection: {}", e),
                };
                
                let users = "SELECT id, first_name, last_name, age, gender FROM users WHERE ID > ? LIMIT 20"
                .with((from_id,))
                .map(&mut conn, |(id, first_name, last_name, age, gender)| 
                    Registrant { 
                     id, first_name, last_name, gender, age 
                 })
                .await?;

                drop(conn);

                //pool.disconnect().await?;

                Ok(users)
        }

        pub async fn get_user (id: String, pool: Pool) -> Result<Registrant, Error> {
            let mut conn = pool.get_conn().await?; // ? is the same as match Ok(conn) => conn, Err(e) => return Err(e)

            let user: Vec<Registrant>= "SELECT ID, signature FROM users WHERE ID = ?"
            .with((id,))
            .map(&mut conn, |(id, first_name, last_name, gender, age )| 
            Registrant { 
                id, first_name, last_name, gender, age 
            })
            .await?;

            let get_single_user = user.get(0).cloned();

    
                let single_user: Registrant = match get_single_user {
                    Some(user) => user,
                    None => Registrant {
                        id: Some(0),
                        first_name: "no user found with that id".into(),
                        last_name: "none".into(),
                        age: 0,
                        gender: 0
                    },
                    };
          
            
             println!("{:?}", user);
             drop(conn);

             Ok(single_user)

        }
    }

    pub mod post {
        use mysql_async::{prelude::*, Pool, Error};
        use crate::models::Registrant;

            pub async fn new_registration(registrant: Registrant, pool: Pool) -> Result<String, Error> {
                let Registrant {first_name, last_name, age, gender, ..} = registrant;
                let mut conn = pool.get_conn().await?;


                let query = 
                "INSERT INTO registrant (first_name, last_name, gender, age) 
                VALUES (:first_name, :last_name, :gender, :age)";

                let params = params! {
                    "first_name" => first_name,
                    "last_name" => last_name,
                    "gender" => gender,
                    "age" => age,
                };

                match conn.exec_drop(query, params).await? {
                    () => println!("operation was successful"),
                    _ => panic!("couldn't insert new user"),
                };
                

                Ok(String::from("everything good"))
            }

        }
    
}