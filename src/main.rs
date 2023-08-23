use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;
use mysql::serde_json::json; // 用来处理日期

fn main() {
    let url = "mysql://root:tig12@localhost:3306/shanxiJson";
    let pool = Pool::new(url).unwrap(); // 获取连接池
    let mut conn = pool.get_conn().unwrap();// 获取链接

    conn.query_iter("select * from student")
        .unwrap()
        .for_each(|row| {
            let r: (i32, String, i32, String, NaiveDate) = from_row(row.unwrap());
            println!("{}, {},{},{}, {:?}", r.0, r.1, r.2, r.3, r.4);
        });
    // let mut jsonStr = "{"agricultureJosn":{"nysczz":1223,"gdpRatio":"5.4","changeState":"","changeRatio":"8","lszzArea":"4707","lsYield":"1421.2","sgYield":"975","scYield":"862","rlYield":"134.4","rlSelfSufficiencyRate":"95","dYield":"108","nYield":"117","scpYield":"5.09","scpChangeState":"","scpChangeRatio":"9.2"}}";
    let mut jsonStr = "{\"agricultureJosn\":{\"nysczz\":1223,\"gdpRatio\":\"5.4\",\"changeState\":\"\",\"changeRatio\":\"8\",\"lszzArea\":\"4707\",\"lsYield\":\"1421.2\",\"sgYield\":\"975\",\"scYield\":\"862\",\"rlYield\":\"134.4\",\"rlSelfSufficiencyRate\":\"95\",\"dYield\":\"108\",\"nYield\":\"117\",\"scpYield\":\"5.09\",\"scpChangeState\":\"\",\"scpChangeRatio\":\"9.2\"}}";

    // 根据json字符串获取json对象
    let json = json!({
        "agricultureJosn": {
            "nysczz": 1223,
            "gdpRatio": "5.4",
            "changeState": "",
            "changeRatio": "8",
            "lszzArea": "4707",
            "lsYield": "1421.2",
            "sgYield": "975",
            "scYield": "862",
            "rlYield": "134.4",
            "rlSelfSufficiencyRate": "95",
            "dYield": "108",
            "nYield": "117",
            "scpYield": "5.09",
            "scpChangeState": "",
            "scpChangeRatio": "9.2"
        }
    });

    //根据json对象创建建表语句
    let mut sql = "create table if not exists student (id int not null primary key auto_increment, name varchar(20) not null, age int not null, address varchar(20) not null, birthday date not null, jsonStr json not null) engine=InnoDB default charset=utf8;".to_string();
    
}