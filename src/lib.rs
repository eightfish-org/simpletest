#![allow(dead_code)]
use anyhow::Result;
use bytes::Bytes;
use spin_sdk::redis_component;
use std::sync::OnceLock;
// use std::cell::OnceCell;

use eightfish_sdk::{App as EightFishApp, GlobalFilter, Request, Response, Result as EFResult};

mod article;

struct MyGlobalFilter;

impl GlobalFilter for MyGlobalFilter {
    fn before(&self, _req: &mut Request) -> EFResult<()> {
        Ok(())
    }

    fn after(&self, _req: &Request, _res: &mut Response) -> EFResult<()> {
        Ok(())
    }
}

pub fn build_app() -> EightFishApp {
    let mut sapp = EightFishApp::new();
    sapp.add_global_filter(Box::new(MyGlobalFilter))
        .add_module(Box::new(article::ArticleModule));

    sapp
}

// Static Lazy to hold the Worker instance
static WORKER: OnceLock<spin_worker::Worker> = OnceLock::new();

/// Main entry
#[redis_component]
fn on_message(message: Bytes) -> Result<()> {
    let sw = WORKER.get_or_init(|| {
        let app = build_app();
        let mut sw = spin_worker::Worker::mount(app);
        sw.set_on_block_height(|_height, _hash| {
            // println!("block height: {height}");
            // println!("block hash: {hash}");
        });
        sw
    });

    sw.work(message)?;

    Ok(())
}
