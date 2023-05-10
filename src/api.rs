pub mod server {
    use crate::db::{get_connection, with_db};
    use super::routes::{get, post, put};
    use std::collections::HashMap;
    use warp::{Filter};

    pub async fn start() -> Result<(), mysql_async::Error> {

        // get the connection pool
        let pool = get_connection().await?;

        // root path
        let root_path = warp::path("api");

        // stablish routes
       let api_routes = root_path
            .and(warp::get())
            .and(warp::path!("users" / String))
            .and(with_db(pool.clone()))
            .and_then(get::get_user)

            .or(root_path
            .and(warp::get())
            .and(warp::path!("users"))
            .and(warp::query::<HashMap<String, String>>())
            .and(with_db(pool.clone()))
            .and_then(|query:HashMap<String, String>, pool| {
                let query=query.get("fromid").map_or_else(|| String::from("0"), |s| s.to_owned());

                get::get_users_handler(query, pool)
            })
            )
            
            .or(
                root_path
                .and(warp::post())
                .and(warp::path("users"))
                .and(warp::path("new"))
                .and(warp::body::json())
                .and(with_db(pool.clone()))
                .and_then(post::create_registration)
            );

        let public_routes = 
         warp::path::end()
        .and(warp::get())
        .and(warp::fs::dir("src/public/"));


        let routes = public_routes.or(api_routes);

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

        Ok(())
    }
}


mod routes {

        // get routes
        pub mod get {
            use warp::{Reply, Rejection};
            use crate::db::queries::get;
            use mysql_async::{Pool};
            use anyhow::{Result};
                
                // gets all users starting from the id passed in the query
            pub async fn get_users_handler(query: String, pool: Pool) -> Result<impl Reply, Rejection> {
      
                let get_users= get::get_all_users(query, pool).await;
      
                let users = match get_users {
                    Ok(users) => users,
                    Err(e) => panic!("couldn't get users: {}", e),
                };

                let reply = warp::reply::json(&users);
                Ok(reply)
            }

            // get a specific user
            pub async fn get_user(id: String, pool:Pool) -> Result<impl Reply, Rejection>{
                let get_user = get::get_user(id, pool).await;
                let user = match get_user {
                    Ok(user) => user,
                    Err(e) => panic!("couldn't get user: {}", e),
                };

                let reply = warp::reply::json(&user);
                Ok(reply)

            }
        }

        // post routes
        pub mod post {
            use crate::models::Registrant;
            use crate::db::queries::post;
            use warp::{Reply, Rejection};
            use mysql_async::{Pool};
            use anyhow::{Result};


            // register a new user
            pub async fn create_registration(registrant: Registrant, pool: Pool) -> Result<impl Reply, Rejection> {


                let register_user = post::new_registration(registrant, pool).await;

                let result = match register_user {
                    Ok(result)=> result, 
                    Err(e) => panic!("Error! Could not create a new registrant: {}", e),
                };

       
                Ok(result)

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
