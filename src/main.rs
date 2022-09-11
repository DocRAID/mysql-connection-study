use rocket_contrib::json::Json;

#[macro_use]
extern crate rocket;

struct User{
    user_id:i32,
    user_name:String
}
fn connect() -> String{
    let db_user = "root";
    let db_password = "0000";
    let db_address = "localhost";
    let db_port = "3306";
    let db_name = "connectiontest";
    format!("mysql://{}:{}@{}:{}/{}",db_user,db_password,db_address,db_port,db_name)
}

#[get("/")]
fn index() -> String {  
    format!("home")
}
#[get("/<index>")]
fn mysql_connect(index:usize) -> String {

    let db_url:String = connect();

    let pool = mysql::Pool::new(db_url).expect("연결실패");

    let query = "select * from name";
    
    let result = pool.prep_exec(query,()).expect("쿼리 오류");
    let mut vt: Vec<User>=Vec::new();
    for row in result {
        let (id,name):(i32, String) = mysql::from_row(row.unwrap());
        vt.push(User { user_id: id, user_name: name })
    }
    // print!("{} {}",vt[1].user_id,vt[1].user_name);
    format!("{} {}",vt[index].user_id,vt[index].user_name)

}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/",routes![index])
        .mount("/mysql",routes![mysql_connect])
}