use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::sync::Mutex;

// Global state for saved bus stops
struct AppState {
    saved_stops: Mutex<Vec<BusStop>>,
}

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
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

// API Response structures
#[derive(Debug, Serialize, Deserialize)]
struct BusLocationResponse {
    result: String,
    busstop_lastdata: Vec<BusStopLastData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BusStopLastData {
    station_id: i32,
    station_name: String,
    company_id: i32,
    bus_list: Vec<BusData>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BusData {
    #[serde(default)]
    bus_id: String,
    #[serde(default)]
    line_id: i32,
    #[serde(default)]
    line_name: String,
    #[serde(default)]
    course_id: i32,
    #[serde(default)]
    course_name: String,
    #[serde(default)]
    time: String,
    #[serde(default)]
    delay_time: String,
    #[serde(default)]
    congestion: String,
    #[serde(default)]
    lat: String,
    #[serde(default)]
    lon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusSearchRequest {
    pub start_station_id: Option<String>,
    pub end_station_id: Option<String>,
    pub destination: Option<String>,
    pub time_from: Option<String>,
    pub time_to: Option<String>,
}

// Data structures for Station/Transit API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StationPrediction {
    pub station_id: i32,
    pub company_id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StationPredictionResponse {
    pub result: String,
    pub route_prediction: Vec<StationPrediction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Station {
    pub station_id: i32,
    pub name: String,
    pub phonic: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub line_id: i32,
    pub course_id: i32,
    pub course_name: String,
    pub station_id: i32,
    pub station_name: String,
    pub pos: i32,
    pub st_flag: i32,
    pub stop_no: String,
    pub company_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StationLine {
    pub line_id: i32,
    pub line_name: String,
    pub station_id: i32,
    pub station_name: String,
    pub company_id: i32,
    pub course_list: Vec<Course>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StationLineListResponse {
    pub result: String,
    pub station_linelist: Vec<StationLine>,
    pub station_list: Vec<Station>,
}

// Route Search API structures (for Get_search_route)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteListItem {
    pub line_id: i64,
    pub line_name: String,
    pub company_id: String,
    pub from_id: i64,
    pub from_name: String,
    pub to_id: i64,
    pub to_name: String,
    pub sec_time: i32,
    pub fare: String,
    pub c_fare: i32,
    pub line_type: i32,
    pub connect_time: i32,
    pub connect_distance: i32,
    pub connect_flag: i32,
    pub service_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalkData {
    pub start_walk_time: i32,
    pub start_walk_dist: i32,
    pub goal_walk_time: i32,
    pub goal_walk_dist: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteInfo {
    pub total_time: i32,
    pub total_connect_time: i32,
    pub line_count: i32,
    pub fare: i32,
    pub c_fare: i32,
    pub route_list: Vec<RouteListItem>,
    pub pattern: String,
    pub service_count: i32,
    pub service_count_raw: i32,
    pub service_frequency: i32,
    pub walk_data: WalkData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRouteResponse {
    pub result: String,
    pub search_route: SearchRouteData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRouteData {
    pub route: Vec<RouteInfo>,
}

// Tauri commands
#[tauri::command]
async fn search_buses(request: BusSearchRequest) -> Result<Vec<BusInfo>, String> {
    let client = reqwest::Client::new();

    // If station IDs are provided, use Get_search_route API
    if let (Some(start_id), Some(end_id)) = (&request.start_station_id, &request.end_station_id) {
        let params = [
            ("kind", "0"),
            ("start_st", start_id.as_str()),
            ("end_st", end_id.as_str()),
            ("lat1", "-1"),
            ("lon1", "-1"),
            ("lat2", "-1"),
            ("lon2", "-1"),
            ("bus_prediction_flg", "1"),
            ("sort_id", "1"),
            ("lang", ""),
            ("start_flg", "0"),
            ("end_flg", "0"),
        ];

        let response = client
            .post("https://ekibus-api.city.sapporo.jp/Get_search_route")
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch routes: {}", e))?;

        let route_data: SearchRouteResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        // Convert route data to BusInfo
        let mut buses = Vec::new();

        for route in route_data.search_route.route {
            for segment in route.route_list {
                let bus_info = BusInfo {
                    bus_id: format!("{}", segment.line_id),
                    route_name: segment.line_name.clone(),
                    destination: segment.to_name.clone(),
                    stop_id: segment.from_id.to_string(),
                    stop_name: segment.from_name.clone(),
                    arrival_time: None, // Route search doesn't provide arrival times
                    delay_minutes: None,
                    congestion_level: None,
                    updated_at: chrono::Local::now().to_rfc3339(),
                    latitude: None,
                    longitude: None,
                };
                buses.push(bus_info);
            }
        }

        return Ok(buses);
    }

    // Fallback to original bus location API
    let body = serde_json::json!({
        "kind": "0",
        "lang": ""
    });

    let response = client
        .post("https://ekibus-api.city.sapporo.jp/Get_busstop_lastdata")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch bus data: {}", e))?;

    let data: BusLocationResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Convert API response to BusInfo
    let mut buses = Vec::new();

    for stop_data in data.busstop_lastdata {
        for bus in stop_data.bus_list {
            // Parse coordinates
            let latitude = bus.lat.parse::<f64>().ok();
            let longitude = bus.lon.parse::<f64>().ok();

            // Parse delay time
            let delay_minutes = if !bus.delay_time.is_empty() {
                bus.delay_time.parse::<i32>().ok()
            } else {
                Some(0)
            };

            // Map congestion level
            let congestion_level = if !bus.congestion.is_empty() {
                Some(bus.congestion.clone())
            } else {
                None
            };

            let bus_info = BusInfo {
                bus_id: if bus.bus_id.is_empty() {
                    format!("{}-{}", bus.line_id, bus.course_id)
                } else {
                    bus.bus_id
                },
                route_name: bus.line_name.clone(),
                destination: bus.course_name.clone(),
                stop_id: stop_data.station_id.to_string(),
                stop_name: stop_data.station_name.clone(),
                arrival_time: if !bus.time.is_empty() {
                    Some(bus.time)
                } else {
                    None
                },
                delay_minutes,
                congestion_level,
                updated_at: chrono::Local::now().to_rfc3339(),
                latitude,
                longitude,
            };

            // Filter by destination if provided
            if let Some(ref dest) = request.destination {
                if bus_info.destination.contains(dest) || bus_info.route_name.contains(dest) {
                    buses.push(bus_info);
                }
            } else {
                buses.push(bus_info);
            }
        }
    }

    // Filter by time if provided
    if let Some(time_from) = request.time_from {
        buses.retain(|bus| {
            if let Some(ref arrival) = bus.arrival_time {
                arrival >= &time_from
            } else {
                false
            }
        });
    }

    Ok(buses)
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
async fn search_station_suggestions(search_word: String) -> Result<Vec<StationPrediction>, String> {
    let client = reqwest::Client::new();

    let mut params = std::collections::HashMap::new();
    params.insert("kind", "0");
    params.insert("search_flg", "1");
    params.insert("search_word", &search_word);
    params.insert("pos_flg", "0");
    params.insert("lang", "");

    let response = client
        .post("https://ekibus-api.city.sapporo.jp/get_route_prediction")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch station suggestions: {}", e))?;

    let data: StationPredictionResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data.route_prediction)
}

#[tauri::command]
async fn get_station_timetable(station_id: i32) -> Result<StationLineListResponse, String> {
    let client = reqwest::Client::new();

    let station_id_str = station_id.to_string();
    let mut params = std::collections::HashMap::new();
    params.insert("kind", "0");
    params.insert("station_id", station_id_str.as_str());
    params.insert("lang", "");

    let response = client
        .post("https://ekibus-api.city.sapporo.jp/Get_station_linelist")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch station timetable: {}", e))?;

    let data: StationLineListResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data)
}

#[tauri::command]
async fn save_bus_stop(
    state: tauri::State<'_, AppState>,
    stop: BusStop,
) -> Result<Vec<BusStop>, String> {
    let mut stops = state
        .saved_stops
        .lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;

    // Check if stop already exists
    if !stops.iter().any(|s| s.stop_id == stop.stop_id) {
        stops.push(stop);
    }

    Ok(stops.clone())
}

#[tauri::command]
async fn get_saved_bus_stops(state: tauri::State<'_, AppState>) -> Result<Vec<BusStop>, String> {
    let stops = state
        .saved_stops
        .lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;

    Ok(stops.clone())
}

#[tauri::command]
async fn delete_bus_stop(
    state: tauri::State<'_, AppState>,
    stop_id: String,
) -> Result<Vec<BusStop>, String> {
    let mut stops = state
        .saved_stops
        .lock()
        .map_err(|e| format!("Failed to lock state: {}", e))?;

    stops.retain(|s| s.stop_id != stop_id);

    Ok(stops.clone())
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
        .manage(AppState {
            saved_stops: Mutex::new(Vec::new()),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_sql::Builder::default().build())
        .plugin(tauri_plugin_http::init())
        .invoke_handler(tauri::generate_handler![
            search_buses,
            get_bus_stops,
            fetch_bus_location_data,
            search_station_suggestions,
            get_station_timetable,
            save_bus_stop,
            get_saved_bus_stops,
            delete_bus_stop,
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
