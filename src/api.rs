pub mod server {
    use warp::{Filter,  http::{Response, StatusCode}};
    use super::routes::{get, post, put};
    use crate::db::{get_connection, with_db};
    use std::collections::HashMap;

    pub async fn start() -> Result<(), mysql_async::Error> {

        // get the connection pool
        let pool = get_connection().await?;

        // root path
        let root_path = warp::path("api");

        // stablish routes
       let routes = root_path
            .and(warp::get())
            .and(warp::path("users"))
            .and(warp::query::<HashMap<String, String>>())
            .and(with_db(pool.clone()))
            .and_then(|query:HashMap<String, String>, pool| {
                let query = match query.get("fromid") {
                    Some(query) => query,
                    None =>  panic!("couldn't get query fromid"),
                };

                get::get_users_handler(query.clone(), pool)
            });
            //.and(warp::path::end());
            // .or(
            //     root_path
            //         .and(warp::get())
            //         .and(warp::path("users"))
            //         .and(warp::path::param::<u64>())
            //         .and_then(|id| get::get_user(id))
            // );

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

        Ok(())
    }
}


mod routes {

        // get routes
        pub mod get {
            use warp::{Reply, Rejection};
            use mysql_async::{Pool};
            use anyhow::{Result};
            use crate::db;

                
            pub async fn get_users_handler(query: String, pool: Pool) -> Result<impl Reply, Rejection> {
      
                let get_users= db::queries::get::get_all_users(query, pool).await;
      
                let users = match get_users {
                    Ok(users) => users,
                    Err(e) => panic!("couldn't get users: {}", e),
                };

                let reply = warp::reply::json(&users);
                Ok(reply)
            }

            // get a specific user
            pub async fn get_user(user: u64) -> Result<String, mysql_async::Error>{
                let user = user.to_string();
                let response = String::from("this user is: ") + &user;

                Ok(response)

            }
        }

        // post routes
        pub mod post {
            use warp::{Reply, Rejection};

            // register a new user
            pub async fn register_user() -> Result<impl Reply, Rejection> {
                let response = String::from("...registering user");
                Ok(response)

            }
        }

        // edit routes  
        pub mod put{
              use warp::{Reply, Rejection};
            // check in the registered user
            pub async fn check_in_user(user: u64) -> Result<impl Reply, Rejection> {
                let user = user.to_string();
                let response = String::from("this user is: ") + &user;

                Ok(response)
            }

            // check out the registered user
            pub async fn check_out_user(user: u64) -> Result<impl Reply, Rejection> {
                let user = user.to_string();
                let response = String::from("this user is: ") + &user;

                Ok(response)
            } 
        }
}
