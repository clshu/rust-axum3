#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::{json, Value};

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
	req_login.await?.print().await?;

	let req_create_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "create_task",
			"params": {
				"data": {
					"title": "task AAA"
				}
			}
		}),
	);

	req_create_task.await?.print().await?;

	let req_list_tasks = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks"
		}),
	);
	req_list_tasks.await?.print().await?;

	let req_update_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "update_task",
			"params": {
				"id": 1008, // Hardcode the task id.
				"data": {
					"title": "task BB"
				}
			}
		}),
	);
	req_update_task.await?.print().await?;

	let req_list_tasks1 = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks"
		}),
	);
	req_list_tasks1.await?.print().await?;

	let req_delete_task = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "delete_task",
			"params": {
				"id": 1008 // Harcode the task id
			}
		}),
	);
	req_delete_task.await?.print().await?;

	let req_list_tasks2 = hc.do_post(
		"/api/rpc",
		json!({
			"id": 1,
			"method": "list_tasks"
		}),
	);
	req_list_tasks2.await?.print().await?;

	let req_logoff = hc.do_post(
		"/api/logoff",
		json!({
			"logoff": true
		}),
	);
	req_logoff.await?.print().await?;

	Ok(())
}
