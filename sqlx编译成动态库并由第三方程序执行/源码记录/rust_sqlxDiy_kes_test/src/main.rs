extern crate sqlx;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgRow;
use std::env;
use rand::Rng;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::postgres::types::*;
use rust_decimal::prelude::*;
use std::ops::Bound::*;
use std::ops::RangeBounds;
use std::fmt;
use serde_json::{Value};


#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
	println!("程序执行完毕");

    let connection_str = env::var("DATABASE_URL").expect("数据库连接字符串获取失败，请检查env文件是否已配置数据库连接字符串");
    println!("数据库连接字符串是：{}", connection_str);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        // .connect("kingbase://system:123456@127.0.0.1:54321/test")
        .connect(&connection_str)
        .await?;
    println!("db_pool is : {:?}", pool);
	let mut s_rng = rand::thread_rng();
/*
	//增加数据给表json
	for i in 1..10{
		let jsonval = r#"
			{
			"article": "how to work with json in Rust",
			"author": "tdep",
			"paragraph": [
				{
				"name": "untyped"
				},
				{
				"name": "strongly typed"
				},
				{
				"name": "writing json"
				}
			]
			}
			"#;
		let parsed: Value = serde_json::from_str(jsonval).unwrap();
		let insert = sqlx::query!(r#"INSERT INTO testjson VALUES ($1, $2)"#, i, parsed: Value).fetch_all(&pool).await?;
	  }

	  //查看json表所有数据
	let list = sqlx::query( "select * from testjson").fetch_all(&pool).await?;
    for row in list {
        let  id:_ = sqlx::Row::get::<i64,  _>(&row, 0);
		let jsonName: Value = sqlx::Row::get(&row, 1);
		let  jsonVal = serde_json::to_string(&jsonName).unwrap();
		println!("{}, {}", id, jsonVal);
    }


	//增加数据给表rangint
	for i in 1..10{
		let  start = NaiveDate::from_ymd_opt(2015, 3, 14);
		let end = NaiveDate::from_ymd_opt(2015, 12, 14);
		let  t2 = PgRange{start: Included(1), end: Included(13)};
		let insert = sqlx::query!(r#"INSERT INTO rangint VALUES ($1, $2)"#, i, t2).fetch_all(&pool).await?;
	  }
*/
	  //查看rangeint表所有数据
/*	let list = sqlx::query!("select * from rangint").fetch_all(&pool).await?;
    for row in list {
        let  id= row.id;
        let rangeVal = row.rangeVal.unwrap();
		println!("{}, {}", id, rangeVal);
    }*/

/*
	  //增加数据给表course
	  for i in 1..1000{
		let  name = format!("name_{}", i);
		let  t4 = NaiveDate::from_ymd_opt(2015, 3, 14);
		let  t5 = 10.1;
		let  t6 = "first line text";
		let  t7 = 0.2;
		let  t8 = "D";
		//let  t9 = PgMoney::from_decimal(Decimal::new(3141, 3), 2);
		let  t9 = PgMoney(10);
		let  t10 = true;
		let insert = sqlx::query!(r#"INSERT INTO course VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#, i, s_rng.gen_range(1..=20), name, t4, t5, t6, t7, t8, t9, t10).fetch_all(&pool).await?;
	  }

	  //增加数据给表teacher
	  for i in 1..21{
		let  name = format!("name_{}", i);
		let insert = sqlx::query!(r#"INSERT INTO teacher VALUES ($1, $2)"#, i, name).fetch_all(&pool).await?;
	  }

    //查询所有
    let list = sqlx::query!("select * from course").fetch_all(&pool).await?;
    let mut vec = vec![];
    for row in list {
        vec.push(Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: row.time,
			score: row.score.unwrap(),
    	    profile: row.profile.unwrap(),
  	        basePoint: row.basePoint.unwrap(),
	        code: row.code.unwrap(),
	        cost: row.cost.unwrap(),
			state: row.state.unwrap(),
        })
    }
    println!("数据库中的所有数据：{:#?}", vec);

    //查询单个
    let list2 = sqlx::query!(r#"select * from course where id = $1"#, 1)
        .fetch_all(&pool)
        .await?;
    let mut vec2 = vec![];
    for row in list2 {
        vec2.push(Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: row.time,
			score: row.score.unwrap(),
    	    profile: row.profile.unwrap(),
  	        basePoint: row.basePoint.unwrap(),
	        code: row.code.unwrap(),
	        cost: row.cost.unwrap(),
			state: row.state.unwrap(),
        })
    }
    println!("查询单个{:#?}", vec2);

    //更新
    let update = sqlx::query!(r#"update  course set name=$1 where teacher_id=$2"#, "ogg", rand::thread_rng().gen_range(1..=11))
        .fetch_all(&pool)
        .await?;

	//查询join
	let list2 = sqlx::query!("select course.id,course.teacher_id,course.name,course.time  from course join teacher on course.teacher_id=teacher.id")
        .fetch_all(&pool)
        .await?;

    for row in list2 {
		println!("join查询结果：{}, {}, {}, {}", row.id.unwrap(), row.teacher_id.unwrap(),  row.name.unwrap(), row.time.unwrap());
    }

	//查询join+where
	let list2 = sqlx::query!("select course.id,course.teacher_id,course.name,course.time from course join teacher on (teacher.id=course.teacher_id) where course.id = 10")
	.fetch_all(&pool)
	.await?;
	for row in list2 {
		println!("join+where查询结果：{}, {}, {}, {}", row.id.unwrap(), row.teacher_id.unwrap(),  row.name.unwrap(), row.time.unwrap());
	}

	//删除表中指定行数据
	let delete = sqlx::query!(r#"delete  from  course where id=$1"#, 10)
        .fetch_all(&pool)
        .await?;

	//删除表
	let delete = sqlx::query!("drop table course").fetch_all(&pool).await?;
	let delete = sqlx::query!("drop table teacher").fetch_all(&pool).await?;
*/
    Ok(())
}
/*
#[derive(Debug)]
pub struct Course {
    pub id: i64,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDate>,
	pub  score:  f64,
    pub profile: String,
    pub  basePoint:  f64,
    pub code:  String,
    pub  cost:  PgMoney,
    pub state:  bool
}*/


