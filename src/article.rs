use anyhow::{anyhow, bail};
use eightfish_derive::EightFishModel;
use eightfish_sdk::{EightFishModel, Module, Request, Response, Result, Router, Status};
use http::HeaderName;
use serde::{Deserialize, Serialize};
use serde_json::json;
use spin_sdk::pg::{DbValue, Decode, ParameterValue};
use spin_worker::{
    sql_create_one, sql_delete, sql_delete_one, sql_query, sql_query_one, sql_update,
    sql_update_one,
};

const REDIS_URL: &str = "REDIS_URL";
const DB_URL: &str = "DB_URL";

#[derive(Debug, Clone, Serialize, Deserialize, EightFishModel, Default)]
pub struct Article {
    id: String,
    title: String,
    content: String,
    authorname: String,
}

pub struct ArticleModule;

impl ArticleModule {
    fn get_one(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;
        let article_id = params.get("id").ok_or(anyhow!("id error"))?;
        println!("article_id: {}", article_id);

        let article = sql_query_one!(Article, article_id);
        println!("article: {:?}", article);
        let results: Vec<Article> = if let Some(article) = article {
            vec![article]
        } else {
            bail!("no this item".to_string());
        };

        Ok(Response::new(Status::Successful, results))
    }

    fn new_article(req: &mut Request) -> Result<Response> {
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

        // let ret = sql_query!(Article, "", &[]);
        // let ret = sql_delete!(req, Article, "", &[]);
        // let ret = sql_update!(req, Article, "", &[]);
        let ret = sql_create_one!(req, article);

        if let Ok(article) = ret {
            Ok(Response::new(Status::Successful, vec![article]))
        } else {
            // example for return a failed json response, with a custom header setting
            let json_result = json!({
                "status": "failed",
                "info": "error when create a new article",
            });
            let mut res = Response::from_failed(json_result);
            res.set_header(
                HeaderName::from_static("xxx-yyy-zzz"),
                "whatever".parse().unwrap(),
            );
            Ok(res)
        }
    }

    fn update(req: &mut Request) -> Result<Response> {
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

        let old_article = sql_query_one!(Article, &id);
        match old_article {
            Some(old_article) => {
                let article = Article {
                    id,
                    title,
                    content,
                    authorname,
                    ..old_article
                };

                if let Ok(instance) = sql_update_one!(req, article) {
                    let ret = vec![instance];
                    Ok(Response::new(Status::Successful, ret))
                } else {
                    bail!("update action: db operation error.")
                }
            }
            None => {
                bail!("update action: no item in db")
            }
        }
    }

    fn delete(req: &mut Request) -> Result<Response> {
        let params = req.parse_urlencoded()?;

        let id = params.get("id").ok_or(anyhow!("id error"))?.to_owned();

        // ensure there is the target item in db
        if let Some(article) = sql_query_one!(Article, &id) {
            if let Ok(instance) = sql_delete_one!(req, article) {
                let ret = vec![instance];
                Ok(Response::new(Status::Successful, ret))
            } else {
                bail!("delete action: error in db")
            }
        } else {
            bail!("delete action: no item in db")
        }
    }

    fn version(_req: &mut Request) -> Result<Response> {
        let ret = r#"{"version": 1.19}"#.to_string();
        let response = Response::from_str(Status::Successful, ret);

        Ok(response)
    }
}

impl Module for ArticleModule {
    fn router(&self, router: &mut Router) -> Result<()> {
        router.get("/simpletest/v1/article", Self::get_one);
        router.post("/simpletest/v1/article/new", Self::new_article);
        router.put("/simpletest/v1/article/update", Self::update);
        router.delete("/simpletest/v1/article/delete", Self::delete);

        router.get("/simpletest/v1/version", Self::version);

        Ok(())
    }
}
