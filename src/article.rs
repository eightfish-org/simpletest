use anyhow::{anyhow, bail};
use eightfish_derive::EightFishModel;
use eightfish_sdk::{
    EightFishModel, HandlerCRUD, Info, Module, Request, Response, Result, Router, Status,
};
use serde::{Deserialize, Serialize};
use spin_sdk::pg::{self, DbValue, Decode, ParameterValue};

const REDIS_URL_ENV: &str = "REDIS_URL_ENV";
const DB_URL_ENV: &str = "DB_URL_ENV";

#[derive(Debug, Clone, Serialize, Deserialize, EightFishModel, Default)]
pub struct Article {
    id: String,
    title: String,
    content: String,
    authorname: String,
}

pub struct ArticleModule;

impl ArticleModule {
    fn get_one(req: &mut Request) -> Result<Response<Article>> {
        let params = req.parse_urlencoded()?;
        let article_id = params.get("id").ok_or(anyhow!("id error"))?;

        let article = sql_query_one!(Article, article_id);
        let results = if let Some(article) = article {
            vec![article]
        } else {
            bail!("no this item".to_string());
        };

        Ok(Response::new(Status::Successful, results))
    }

    fn new_article(req: &mut Request) -> Result<Response<Article>> {
        let params = req.parse_urlencoded()?;
        let title = params
            .get("title")
            .ok_or(anyhow!("title error"))?
            .to_owned();
        let content = params
            .get("content")
            .ok_or(anyhow!("content error"))?
            .to_owned();
        let authorname = params
            .get("authorname")
            .ok_or(anyhow!("authorname error"))?
            .to_owned();
        let id = req
            .ext()
            .get("random_str")
            .ok_or(anyhow!("id error"))?
            .to_owned();

        let article = Article {
            id,
            title,
            content,
            authorname,
        };

        if let Err(_) = sql_create_one!(article) {
            // handle error
        }
        let ret = vec![article];

        Ok(Response::new(Status::Successful, ret))
    }

    fn update(req: &mut Request) -> Result<Response<Article>> {
        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id error"))?.to_owned();
        let title = params
            .get("title")
            .ok_or(anyhow!("title error"))?
            .to_owned();
        let content = params
            .get("content")
            .ok_or(anyhow!("content error"))?
            .to_owned();
        let authorname = params
            .get("authorname")
            .ok_or(anyhow!("authorname error"))?
            .to_owned();

        let old_article = sql_query_one!(Article, id);
        match old_article {
            Some(old_article) => {
                let article = Article {
                    id,
                    title,
                    content,
                    authorname,
                    ..old_article
                };

                if let Err(_) = sql_update_one!(article) {
                    // handle error
                }
                let ret = vec![article];

                Ok(Response::new(Status::Successful, ret))
            }
            None => {
                bail!("update action: no item in db")
            }
        }
    }

    fn delete(req: &mut Request) -> Result<Response<Article>> {
        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id error"))?.to_owned();
        
        // ensure there is the target item in db
        let article: Article = sql_query_one!(Article, id);

        if let Err(_) = sql_delete_one!(article) {
            // handle error
        }
        let ret = vec![article];

        Ok(Response::new(Status::Successful, ret))

        // let (sql, sql_params) = Article::build_delete(id.as_str());
        // _ = pg_conn.execute(&sql, &sql_params)?;

        // let info = Info {
        //     model_name: Article::model_name(),
        //     action: HandlerCRUD::Delete,
        //     extra: "".to_string(),
        // };
        // let results: Vec<Article> = vec![];

        // Ok(Response::new(Status::Successful, info, results))
    }

    fn version(_req: &mut Request) -> Result<Response> {
        let ret = r#"{"version": 1.3}"#.to_string();
        let response = Response::from_str(Status::Successful, ret);

        Ok(response)
    }
}

impl Module for ArticleModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/simpletest/v1/article", Self::get_one);
        router.post("/simpletest/v1/article/new", Self::new_article);
        router.post("/simpletest/v1/article/update", Self::update);
        router.post("/simpletest/v1/article/delete", Self::delete);

        router.get("/simpletest/v1/version", Self::version);

        Ok(())
    }
}
