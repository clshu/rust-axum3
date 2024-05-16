#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	// hc.do_get("/index.html").await?.print().await?;

	let req_login = hc.do_post(
		"/api/login",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	);

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	req_login.await?.print().await?;

	let req_create_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_task",
			"params": {
				"data": {
					"title": "title AAA"
				}
			}
		}),
	);

	req_create_task.await?.print().await?;

	let req_list = hc.do_post(
		"/api/rpc",
		json!({
			"id": 2,
			"method": "list_tasks"
		}),
	);
	req_list.await?.print().await?;

	let req_update_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_task",
			"params": {
				"id": 1000,
				"data": {
					"title": "title DDD"
				}
			}
		}),
	);

	req_update_task.await?.print().await?;

	let req_list2 = hc.do_post(
		"/api/rpc",
		json!({
			"id": 2,
			"method": "list_tasks"
		}),
	);
	req_list2.await?.print().await?;

	let req_delete_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_task",
			"params": {
				"id": 1004,
			}
		}),
	);
	req_delete_task.await?.print().await?;

	let req_list3 = hc.do_post(
		"/api/rpc",
		json!({
			"id": 2,
			"method": "list_tasks"
		}),
	);
	req_list3.await?.print().await?;

	req_logoff.await?.print().await?;

	// hc.do_get("/hello").await?.print().await?;

	Ok(())
}
