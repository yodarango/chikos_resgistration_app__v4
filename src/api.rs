pub mod server {
    use warp::{Filter};
    use super::routes;

    pub async fn start() {

// root path
        let root_path = warp::path("api");
        let routes = root_path
            .and(warp::get())
            .and(warp::path("users"))
            .and(warp::path::end())
            .and_then(routes::get_all_users)
            .or(
                root_path
                    .and(warp::get())
                    .and(warp::path("users"))
                    .and(warp::path::param::<u64>())
                    .and_then(|id| routes::get_user(id))
            );


        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    }
}

mod routes {
        use warp::{Reply, Rejection};

        // get all users
        pub async fn get_all_users() -> Result<impl Reply, Rejection> {
            let response = String::from("...users");
            Ok(response)

        }

        // get a specific user
        pub async fn get_user(user: u64) -> Result<impl Reply, Rejection> {
            let user = user.to_string();
            let response = String::from("this user is: ") + &user;

            Ok(response)

        }
}

