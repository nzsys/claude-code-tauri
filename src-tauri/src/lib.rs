use serde::{Deserialize, Serialize};
use tauri::Manager;

// Data structures for Bus API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusStop {
    pub stop_id: String,
    pub stop_name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusInfo {
    pub bus_id: String,
    pub route_name: String,
    pub destination: String,
    pub stop_id: String,
    pub stop_name: String,
    pub arrival_time: Option<String>,
    pub delay_minutes: Option<i32>,
    pub congestion_level: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusSearchRequest {
    pub destination: Option<String>,
    pub time_from: Option<String>,
    pub time_to: Option<String>,
}

// Tauri commands
#[tauri::command]
async fn search_buses(_request: BusSearchRequest) -> Result<Vec<BusInfo>, String> {
    // For now, return mock data
    // TODO: Implement actual API call
    let mock_buses = vec![
        BusInfo {
            bus_id: "1".to_string(),
            route_name: "中央線".to_string(),
            destination: "札幌駅".to_string(),
            stop_id: "001".to_string(),
            stop_name: "大通駅".to_string(),
            arrival_time: Some("10:30".to_string()),
            delay_minutes: Some(2),
            congestion_level: Some("中".to_string()),
            updated_at: chrono::Local::now().to_rfc3339(),
        },
    ];
    Ok(mock_buses)
}

#[tauri::command]
async fn get_bus_stops() -> Result<Vec<BusStop>, String> {
    // Mock data for now
    let mock_stops = vec![
        BusStop {
            stop_id: "001".to_string(),
            stop_name: "札幌駅".to_string(),
            latitude: Some(43.0686),
            longitude: Some(141.3506),
        },
        BusStop {
            stop_id: "002".to_string(),
            stop_name: "大通駅".to_string(),
            latitude: Some(43.0618),
            longitude: Some(141.3563),
        },
    ];
    Ok(mock_stops)
}

#[tauri::command]
async fn fetch_bus_location_data(
    stop_id_list: Option<Vec<String>>,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    // Build request body
    let mut body = serde_json::json!({});
    if let Some(ids) = stop_id_list {
        body["station_id_list"] = serde_json::json!(ids);
    }

    // Make API request
    let response = client
        .post("https://ekibus-api.city.sapporo.jp/Get_busstop_lastdata")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch bus data: {}", e))?;

    let data = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(data)
}

#[tauri::command]
async fn init_database(app_handle: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            search_buses,
            get_bus_stops,
            fetch_bus_location_data,
            init_database
        ])
        .setup(|app| {
            // Initialize database on startup
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let _ = init_database(app_handle).await;
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
