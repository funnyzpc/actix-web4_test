use serde::{Deserialize, Serialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct ResultBuild<T> {
    pub status: i32,
    pub timestamp: i64,
    // 可选参数
    pub msg: String,
    pub data: T,
}

impl<T>  ResultBuild<T>{
    #[allow(dead_code)]
    // pub fn success() -> Self {
    pub fn success() -> ResultBuild<Option<bool>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:1,timestamp:seconds,msg:String::from("success"),data:None}
    }
    #[allow(dead_code)]
    pub fn success_with_msg(msg:String) -> ResultBuild<Option<bool>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:1,timestamp:seconds,msg:msg,data:None}
    }

    #[allow(dead_code)]
    // pub fn success_with_data(data_list:Vec<T>) -> Self {
    pub fn success_with_data(data:Vec<T>) -> ResultBuild<Vec<T>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:1,timestamp:seconds,msg:String::from("success"),data:data}
    }

    #[allow(dead_code)]
    // pub fn success_with_data(data_list:Vec<T>) -> Self {
    pub fn success_with_data_str(data:&str) -> ResultBuild<&str> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:1,timestamp:seconds,msg:String::from("success"),data:data}
    }

    #[allow(dead_code)]
    // pub fn fail() -> Self {
    pub fn fail() -> ResultBuild<Option<bool>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:0,timestamp:seconds,msg:String::from("fail"),data:None}
    }
    #[allow(dead_code)]
    pub fn fail_with_msg(msg:&str) -> ResultBuild<Option<i8>> {
        let seconds = chrono::offset::Local::now().timestamp();
        ResultBuild {status:0,timestamp:seconds,msg:msg.to_string(),data:None}
    }

}
