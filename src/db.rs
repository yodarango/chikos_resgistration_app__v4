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
        use crate::models::{Guardian,Registrant};

        pub async fn get_all_users(from_id: String, pool: Pool) -> Result<Vec<Registrant>, Error> {
        
                let mut conn = match pool.get_conn().await {
                    Ok(conn) => conn,
                    Err(e) => panic!("couldn't get connection: {}", e),
                };
                
                let users = " 
                SELECT r.id, r.first_name, r.last_name, r.gender, r.age, g.first_name AS guardian_fn, g.last_name AS guardian_ln, g.phone_number AS guardian_phone
                FROM registrant AS r
                JOIN guardians AS g
                ON r.id = g.registrant_id 
                WHERE r.id > ? 
                LIMIT 20"
                .with((from_id,))
                .map(&mut conn, |(id, first_name, last_name, gender, age, guardian_fn, guardian_ln, guardian_phone )| 
                    Registrant { 
                        created_date: None,
                        first_name, 
                        last_name, 
                        gender, 
                        age, 
                        id, 
                        guardian: {
                            Some(
                            Guardian {
                                    first_name: guardian_fn,
                                    last_name: guardian_ln,
                                    phone_number: guardian_phone
                                }
                            )
                        }

                })
                .await?;

                drop(conn);

                //pool.disconnect().await?;

                Ok(users)
        }

        pub async fn get_user (id: String, pool: Pool) -> Result<Registrant, Error> {
            let mut conn = pool.get_conn().await?; // ? is the same as match Ok(conn) => conn, Err(e) => return Err(e)
             
             let query = 
              "SELECT r.id, r.first_name, r.last_name, r.gender, r.age, g.first_name AS guardian_fn, g.last_name AS guardian_ln, g.phone_number AS guardian_phone
                FROM registrant AS r
                JOIN guardians AS g
                ON r.id = g.registrant_id 
                WHERE r.id = ?".with((id,));
            
            //query
            let user: Vec<Registrant>= query
            .map(&mut conn, |(id, first_name, last_name, gender, age, guardian_fn, guardian_ln, guardian_phone )| 
            Registrant { 
                created_date: None,
                first_name, 
                last_name, 
                gender, 
                age, 
                id, 
                guardian: {
                    Some(
                       Guardian {
                            first_name: guardian_fn,
                            last_name: guardian_ln,
                            phone_number: guardian_phone
                        }
                    )
                },

            })
            .await?;

            let get_single_user = user.get(0).cloned();

    
                let single_user: Registrant = match get_single_user {
                    Some(user) => user,
                    None => Registrant {
                        id: Some(0),
                        first_name: "no user found with that id".into(),
                        last_name: "none".into(),
                        created_date: None,
                        guardian: None,
                        gender: 0,
                        age: 0,
                    },
                    };
          
             drop(conn);

             Ok(single_user)

        }

        pub async fn get_last_user_id (pool: Pool) -> Result<u64, mysql_async::Error> {
            let mut conn = pool.get_conn().await?;
            let last_id: Vec<u64>  = 
            "SELECT ID FROM registrant ORDER BY ID DESC LIMIT 1"
            .with(())
            .map(&mut conn, |id | id)
            .await?;

            let last_id = last_id.get(0).unwrap_or(&0);
            Ok(*last_id)
        }
    }

    pub mod post {
        use mysql_async::{prelude::*, Pool, Error};
        use crate::models::{Response, Registrant};
        use warp::{reply::json, Reply};
        use super::get::get_last_user_id;
       // use serde_json::json;

            pub async fn new_registration(registrant: Registrant, pool: Pool) -> Result<impl Reply, Error> {
                 let Registrant {first_name, last_name, age, gender, guardian,..} = &registrant;
                 let mut conn = pool.get_conn().await?;

                let guardian_fn: String = guardian.as_ref().map_or("".into(), |g| g.first_name.clone());
                let guardian_ln: String = guardian.as_ref().map_or("".into(), |g| g.last_name.clone());
                let guardian_phone: u64 = guardian.as_ref().map_or(0, |g| g.phone_number.clone());

                let query = 
                "INSERT INTO registrant (first_name, last_name, gender, age) 
                VALUES (:first_name, :last_name, :gender, :age);";

                let params = params! {
                    "first_name" => first_name,
                    "last_name" => last_name,
                    "gender" => gender,
                    "age" => age,
                };

                  let g_query = "INSERT INTO guardians (registrant_id, first_name, last_name, phone_number)
                VALUES (:registrant_id, :guardian_fn, :guardian_ln, :guardian_phone);
                ";

                 conn.exec_drop(query, params).await?;

                // get the last id after insertion
                let last_id = get_last_user_id(pool).await?;

                let g_params = params! {
                    "registrant_id" => last_id,
                    "guardian_fn" => guardian_fn,
                    "guardian_ln" => guardian_ln,
                    "guardian_phone" => guardian_phone,
                };
                
                let g_result: Response = match conn.exec_drop(g_query, g_params).await? {
                    () => Response {message: "operation was successful".into(), status: 200, data: Some(registrant)},
                };

                Ok(json::<_>(&g_result ))
          
            }

        }
    
}