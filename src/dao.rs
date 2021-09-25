//use diesel::Connection;
extern crate diesel;
//use diesel::query_dsl::methods;
use crate::models::*;
use crate::schema::posts;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::vec::Vec;
use crate::repositories::*;
//use models::Post;

pub struct PostDAO<'a>{
    pub connection: &'a SqliteConnection
}


impl<'a> PostRepository for PostDAO<'a>{

    fn create(&self, new_post: NewPost) -> usize {
        return diesel::insert_into(posts::table)
            .values(new_post)
            .execute(self.connection)
            .expect("作成に失敗");
    }

    fn find_all(&self) -> Vec<Post> {
        return posts::dsl::posts.load::<Post>(self.connection).expect("取得に失敗");
    }

    fn find_one(&self, id: i32) -> Post {
        return posts::dsl::posts.filter(posts::id.eq(id)).first::<Post>(self.connection).expect("取得に失敗");
    }

    fn delete(&self, id: i32) -> Result<(), diesel::result::Error> {
        let result = diesel::delete(posts::dsl::posts.filter(posts::id.eq(id))).execute(self.connection);
        if let Err(err) = result {
            return Err(err);
        };
        return Ok(());
    }
}