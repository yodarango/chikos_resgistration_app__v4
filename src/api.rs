pub mod server {
    use crate::db::{get_connection, with_db};
    use super::routes::{get, post, put, delete, files};
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
            .and(warp::path("users"))
            .and(warp::query::<HashMap<String, String>>())
            .and(with_db(pool.clone()))
            .and_then(|query:HashMap<String, String>, pool| {
                let query=query.get("fromid").map_or_else(|| String::from("0"), |s| s.to_owned());

                get::get_users_handler(query, pool)
            })
            )
            // create new user
            .or(
                root_path
                .and(warp::post())
                .and(warp::path("users"))
                .and(warp::path("new"))
                .and(warp::body::json())
                .and(with_db(pool.clone()))
                .and_then(post::create_registration)
            )
            // check in a user 
            .or(
                root_path
                .and(warp::put())
                .and(warp::path("users"))
                .and(warp::path!("checkin" / u64))
                .and(with_db(pool.clone()))
                .and_then(put::check_in_user)
            )
            // Check out a user
             .or(
                root_path
                .and(warp::put())
                .and(warp::path("users"))
                .and(warp::path!("checkout" / u64))
                .and(with_db(pool.clone()))
                .and_then(put::check_out_user)
            )
            // delete a user 
            .or(
                root_path
                .and(warp::delete())
                .and(warp::path("users"))
                .and(warp::path!("delete" / u64))
                .and(with_db(pool.clone()))
                .and_then(delete::delete_user)
            )
            // file download
            .or(
                root_path
                .and(warp::post())
                .and(warp::path("upload"))
                .and(warp::multipart::form().max_length(5_000_000))
                .and_then(files::upload)
            )
            ;


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
              use mysql_async::{Pool};
              use crate::db::queries::put;
              use anyhow::{Result};

            // check in the registered user
            pub async fn check_in_user(user_id: u64, pool: Pool) -> Result<impl Reply, Rejection> {
                let response = match put::check_in_user(user_id, pool).await {
                    Ok(response) => response,
                    Err(e)=> panic!("Error! Could not check in user: {}", e),
                };

                Ok(response)
            }

           // check in the registered user
            pub async fn check_out_user(user_id: u64, pool: Pool) -> Result<impl Reply, Rejection> {
                let response = match put::check_out_user(user_id, pool).await {
                    Ok(response) => response,
                    Err(e)=> panic!("Error! Could not check out user: {}", e),
                };

                Ok(response)
            }
        }

        // delete queries 
        pub mod delete {
            use mysql_async::{Pool};
            use anyhow::Result;  
            use warp::{Reply, Rejection};
            use crate::db::queries::delete;

            pub async fn delete_user(user_id: u64, pool: Pool) -> Result<impl Reply, Rejection> {
                let response = match delete::delete_user(user_id, pool).await {
                    Ok(response) => response,
                    Err(e) => panic!("Error! Could not delete user: {}", e),
                };
                Ok(response)

            }
        }


        pub mod files {
            use bytes::BufMut;
            use futures::TryStreamExt;
            use uuid::Uuid;
            use warp::{
                multipart::{FormData, Part},
                Rejection, Reply,
            };
            
           pub async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;

    for p in parts {
        if p.name() == "file" {
            let content_type = p.content_type();
            let file_ending;
            match content_type {
                Some(file_type) => match file_type {
                    "application/pdf" => {
                        file_ending = "pdf";
                    }
                    "image/png" => {
                        file_ending = "png";
                    }
                    v => {
                        eprintln!("invalid file type found: {}", v);
                        return Err(warp::reject::reject());
                    }
                },
                None => {
                    eprintln!("file type could not be determined");
                    return Err(warp::reject::reject());
                }
            }

            let value = p
                .stream()
                .try_fold(Vec::new(), |mut vec, data| {
                    vec.put(data);
                    async move { Ok(vec) }
                })
                .await
                .map_err(|e| {
                    eprintln!("reading file error: {}", e);
                    warp::reject::reject()
                })?;

            let file_name = format!("./files/{}.{}", Uuid::new_v4().to_string(), file_ending);
            tokio::fs::write(&file_name, value).await.map_err(|e| {
                eprint!("error writing file: {}", e);
                warp::reject::reject()
            })?;
            println!("created file: {}", file_name);
        }
    }

    Ok("success")
}

            // pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {

            // }
        }
}
