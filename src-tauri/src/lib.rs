use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::sync::Mutex;
use rusqlite::{Connection, params};
use std::path::PathBuf;
use chrono::Datelike;

// Global state for saved bus stops and database
struct AppState {
    saved_stops: Mutex<Vec<BusStop>>,
    db_path: Mutex<Option<PathBuf>>,
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
    pub stops_away: Option<i32>,
    pub estimated_minutes: Option<i32>,
    pub last_stop_name: Option<String>,
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

// Timetable API structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeTableEntry {
    pub from_time: String,
    pub to_time: String,
    pub note: String,
    pub fromto: String,
    pub course_id: i64,
    pub connect_index: i32,
    pub prev_index: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiaList {
    pub dia_flg: i32,
    pub time_table: Vec<TimeTableEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimetableCourse {
    pub course_id: i64,
    pub course_name: String,
    pub line_id: i64,
    pub line_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RouteListForTimetable {
    pub line_id: String,
    pub from_id: String,
    pub to_id: String,
    pub connect_time: i32,
    pub connect_distance: i32,
    pub line_name: String,
    pub from_name: String,
    pub to_name: String,
    pub course_list: Vec<TimetableCourse>,
    pub dia_list: Vec<DiaList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRouteTimetableResponse {
    pub result: String,
    pub search_route_timetable: SearchRouteTimetableData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRouteTimetableData {
    pub time_table: TimeTableInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTableInfo {
    pub from_id: String,
    pub to_id: String,
    pub pattern: String,
    pub from_name: String,
    pub to_name: String,
    pub day_type: i32,
    pub route_list: Vec<RouteListForTimetable>,
}

// Bus Approach Info structures
#[derive(Debug, Serialize, Deserialize)]
pub struct BusApproachInfo {
    pub result: String,
    pub bus_status: i32,
    pub last_stop: i64,
    pub last_stop_order: i32,
    pub delay_time: i32,
    pub last_delay_time: i32,
    pub update_time: String,
    pub barrier_free: i32,
}

// Get_busstop_list structures - This is the key API!
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusTimeInfo {
    pub time: String,
    pub start_time: String,
    pub bus_status: i32,
    pub last_stop: i64,
    pub last_stop_order: i32,
    pub delay_time: i32,
    pub course_id: String,
    pub note: String,
    pub barrier_free: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BusStopListData {
    pub line_id: i64,
    pub line_name: String,
    pub course_id: String,
    pub course_id_name: String,
    pub station_id: String,
    pub station_id_name: String,
    pub pos: i32,
    pub stop_no: String,
    pub terminal_flg: i32,
    pub time_list: Vec<BusTimeInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusStopListResponse {
    pub result: String,
    pub data: Vec<BusStopListData>,
}

// Tauri commands
#[tauri::command]
async fn search_buses(
    request: BusSearchRequest,
    app_handle: tauri::AppHandle,
) -> Result<Vec<BusInfo>, String> {
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

        // Convert route data to BusInfo with enhanced real-time data
        let mut buses = Vec::new();
        let current_time = chrono::Local::now();
        let current_time_str = current_time.format("%H:%M").to_string();

        for route in route_data.search_route.route {
            for segment in route.route_list {
                let mut bus_info = BusInfo {
                    bus_id: format!("{}", segment.line_id),
                    route_name: segment.line_name.clone(),
                    destination: segment.to_name.clone(),
                    stop_id: segment.from_id.to_string(),
                    stop_name: segment.from_name.clone(),
                    arrival_time: None,
                    delay_minutes: None,
                    congestion_level: None,
                    updated_at: chrono::Local::now().to_rfc3339(),
                    latitude: None,
                    longitude: None,
                    stops_away: None,
                    estimated_minutes: None,
                    last_stop_name: None,
                };

                // Only get real-time data for bus segments (line_type == 10 means bus)
                if segment.line_type == 10 {
                    // Use Get_busstop_list to get comprehensive real-time info
                    let busstop_params = [
                        ("kind", "0"),
                        ("line_id", &segment.line_id.to_string()),
                        ("station_id", &segment.from_id.to_string()),
                        ("lang", ""),
                    ];

                    if let Ok(busstop_response) = client
                        .post("https://ekibus-api.city.sapporo.jp/Get_busstop_list")
                        .form(&busstop_params)
                        .send()
                        .await {

                        let response_text = busstop_response.text().await.unwrap_or_default();

                        match serde_json::from_str::<BusStopListResponse>(&response_text) {
                            Ok(busstop_data) => {

                                // Find the first bus arriving after current time
                                for stop_data in &busstop_data.data {
                                        stop_data.line_id, stop_data.station_id, stop_data.time_list.len());

                                    if let Some(next_bus) = stop_data.time_list.iter()
                                        .find(|bus| bus.time >= current_time_str) {

                                            next_bus.time, next_bus.delay_time, next_bus.bus_status);

                                        // Set arrival time
                                        bus_info.arrival_time = Some(next_bus.time.clone());

                                        // Set delay time
                                        bus_info.delay_minutes = Some(next_bus.delay_time);

                                        // Map bus_status to congestion level
                                        bus_info.congestion_level = match next_bus.bus_status {
                                            0 => Some("運行前".to_string()),
                                            1 => Some("空席あり".to_string()),
                                            2 => Some("立席あり".to_string()),
                                            3 => Some("混雑".to_string()),
                                            4 => Some("満員".to_string()),
                                            _ => Some("不明".to_string()),
                                        };

                                        // Update timestamp
                                        bus_info.updated_at = chrono::Local::now().to_rfc3339();

                                        // Calculate stops away (how many stops until arrival)
                                        if next_bus.last_stop_order > 0 {
                                            let stops_remaining = stop_data.pos - next_bus.last_stop_order;
                                            if stops_remaining > 0 {
                                                bus_info.stops_away = Some(stops_remaining);

                                                // Estimate arrival time based on segment time and current position
                                                // Assume uniform time distribution across stops
                                                let total_stops = stop_data.pos; // Total stops in route
                                                if total_stops > 0 {
                                                    let time_per_stop = segment.sec_time / total_stops;
                                                    let estimated_time = (time_per_stop * stops_remaining) / 60; // Convert to minutes
                                                    let estimated_with_delay = estimated_time + next_bus.delay_time;
                                                    bus_info.estimated_minutes = Some(estimated_with_delay.max(0));
                                                }

                                                // Set last stop name (from last_stop ID)
                                                // Note: Would need to fetch station name from API, for now use ID
                                                bus_info.last_stop_name = Some(format!("停留所 #{}", next_bus.last_stop));
                                            }
                                        }

                                        break;
                                    } else {
                                    }
                                }
                            }
                            Err(e) => {
                            }
                        }
                    } else {
                    }

                    // Fallback to timetable if no real-time data available
                    if bus_info.arrival_time.is_none() {

                        // Determine dia_flg based on Sapporo API conventions
                        // Note: Sapporo API uses 1,2,3,4 (not 0,1,2)
                        // We'll try to match by checking all available dia_flg values
                        let preferred_dia_flg = match current_time.weekday() {
                            chrono::Weekday::Sat => 2,  // Assuming 2 = Saturday
                            chrono::Weekday::Sun => 3,  // Assuming 3 = Sunday
                            _ => 1,                      // Assuming 1 = Weekday
                        };

                        // Try to get from cache first
                        let mut timetable_entries = get_cached_timetable(
                            &app_handle,
                            &route.pattern,
                            &segment.line_id.to_string(),
                            preferred_dia_flg,
                        ).ok();

                            timetable_entries.as_ref().map(|e| e.len()).unwrap_or(0));

                        // If not in cache, fetch from API and save to cache
                        if timetable_entries.is_none() || timetable_entries.as_ref().unwrap().is_empty() {
                            let timetable_params = [
                                ("kind", "0"),
                                ("start_station_id", request.start_station_id.as_ref().unwrap().as_str()),
                                ("end_station_id", request.end_station_id.as_ref().unwrap().as_str()),
                                ("time_from", request.time_from.as_ref().unwrap().as_str()),
                                ("pattern", &route.pattern),
                                ("lang", ""),
                            ];

                            if let Ok(timetable_response) = client
                                .post("https://ekibus-api.city.sapporo.jp/Get_search_route_timetable")
                                .form(&timetable_params)
                                .send()
                                .await {

                                if let Ok(timetable_data) = timetable_response.json::<SearchRouteTimetableResponse>().await {
                                        timetable_data.search_route_timetable.time_table.route_list.len());

                                    // Find the matching route segment in the timetable
                                    for route_item in timetable_data.search_route_timetable.time_table.route_list {
                                            route_item.line_id, route_item.dia_list.len());

                                        if route_item.line_id == segment.line_id.to_string() {

                                            // Look through dia_list for time entries
                                            // Try preferred dia_flg first, then fallback to first available
                                            let mut found_dia = None;
                                            for dia in &route_item.dia_list {
                                                    dia.dia_flg, dia.time_table.len());

                                                if dia.dia_flg == preferred_dia_flg {
                                                    found_dia = Some(dia);
                                                        dia.time_table.len());
                                                    break;
                                                }
                                            }

                                            // If preferred dia_flg not found, use first available
                                            if found_dia.is_none() && !route_item.dia_list.is_empty() {
                                                found_dia = Some(&route_item.dia_list[0]);
                                                    route_item.dia_list[0].dia_flg);
                                            }

                                            if let Some(dia) = found_dia {
                                                    dia.time_table.len());

                                                // Save to cache
                                                let _ = save_timetable_to_cache(
                                                    &app_handle,
                                                    &route.pattern,
                                                    &segment.line_id.to_string(),
                                                    &segment.from_id.to_string(),
                                                    &segment.to_id.to_string(),
                                                    dia.dia_flg,
                                                    &dia.time_table,
                                                );

                                                timetable_entries = Some(dia.time_table.clone());
                                            }
                                            break;
                                        }
                                    }
                                } else {
                                }
                            } else {
                            }
                        }

                        // Use the timetable data (from cache or API)
                        // Get all buses within 1 hour from now
                        if let Some(entries) = timetable_entries {
                            let one_hour_later = current_time + chrono::Duration::hours(1);
                            let one_hour_later_str = one_hour_later.format("%H:%M").to_string();

                                entries.len(), current_time_str, one_hour_later_str);

                            let matching_entries: Vec<_> = entries.iter()
                                .filter(|entry| entry.from_time >= current_time_str && entry.from_time <= one_hour_later_str)
                                .collect();


                            for time_entry in matching_entries {
                                let mut bus_copy = bus_info.clone();
                                bus_copy.arrival_time = Some(time_entry.from_time.clone());
                                bus_copy.updated_at = chrono::Local::now().to_rfc3339();
                                buses.push(bus_copy);
                            }
                        } else {
                            buses.push(bus_info);
                        }
                    } else {
                        // Real-time data was found, add the single bus
                        buses.push(bus_info);
                    }
                } else {
                    // Not a bus segment (e.g., walking), just add it
                    buses.push(bus_info);
                }
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
                stops_away: None,
                estimated_minutes: None,
                last_stop_name: None,
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

// Helper function to get database connection
fn get_db_connection(app_handle: &tauri::AppHandle) -> Result<Connection, String> {
    let state = app_handle.state::<AppState>();
    let db_path_lock = state.db_path.lock()
        .map_err(|e| format!("Failed to lock db_path: {}", e))?;

    let db_path = db_path_lock.as_ref()
        .ok_or_else(|| "Database not initialized".to_string())?;

    Connection::open(db_path)
        .map_err(|e| format!("Failed to open database: {}", e))
}

// Get cached timetable data
fn get_cached_timetable(
    app_handle: &tauri::AppHandle,
    route_pattern: &str,
    line_id: &str,
    dia_flg: i32,
) -> Result<Vec<TimeTableEntry>, String> {
    let conn = get_db_connection(app_handle)?;

    let mut stmt = conn.prepare(
        "SELECT from_time, to_time, note, course_id
         FROM timetable_cache
         WHERE route_pattern = ?1 AND line_id = ?2 AND dia_flg = ?3
         ORDER BY from_time"
    ).map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let entries = stmt.query_map(params![route_pattern, line_id, dia_flg], |row| {
        Ok(TimeTableEntry {
            from_time: row.get(0)?,
            to_time: row.get(1)?,
            note: row.get(2)?,
            fromto: "".to_string(),
            course_id: row.get::<_, String>(3)?.parse().unwrap_or(0),
            connect_index: 0,
            prev_index: 0,
        })
    }).map_err(|e| format!("Failed to query timetable: {}", e))?;

    let mut result = Vec::new();
    for entry in entries {
        result.push(entry.map_err(|e| format!("Failed to get entry: {}", e))?);
    }

    Ok(result)
}

// Save timetable data to cache
fn save_timetable_to_cache(
    app_handle: &tauri::AppHandle,
    route_pattern: &str,
    line_id: &str,
    from_id: &str,
    to_id: &str,
    dia_flg: i32,
    entries: &[TimeTableEntry],
) -> Result<(), String> {
    let conn = get_db_connection(app_handle)?;

    // First, delete existing entries for this route
    conn.execute(
        "DELETE FROM timetable_cache
         WHERE route_pattern = ?1 AND line_id = ?2 AND dia_flg = ?3",
        params![route_pattern, line_id, dia_flg],
    ).map_err(|e| format!("Failed to delete old entries: {}", e))?;

    // Insert new entries
    let mut stmt = conn.prepare(
        "INSERT INTO timetable_cache
         (route_pattern, line_id, from_id, to_id, dia_flg, from_time, to_time, course_id, note)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)"
    ).map_err(|e| format!("Failed to prepare insert statement: {}", e))?;

    for entry in entries {
        stmt.execute(params![
            route_pattern,
            line_id,
            from_id,
            to_id,
            dia_flg,
            &entry.from_time,
            &entry.to_time,
            entry.course_id.to_string(),
            &entry.note,
        ]).map_err(|e| format!("Failed to insert entry: {}", e))?;
    }

    Ok(())
}

// Clear all timetable cache (for refresh button)
#[tauri::command]
async fn clear_timetable_cache(app_handle: tauri::AppHandle) -> Result<(), String> {
    let conn = get_db_connection(&app_handle)?;

    conn.execute("DELETE FROM timetable_cache", [])
        .map_err(|e| format!("Failed to clear cache: {}", e))?;

    conn.execute(
        "INSERT OR REPLACE INTO cache_metadata (key, value, updated_at)
         VALUES ('last_cleared', datetime('now'), datetime('now'))",
        [],
    ).map_err(|e| format!("Failed to update metadata: {}", e))?;

    Ok(())
}

#[tauri::command]
async fn init_database(app_handle: tauri::AppHandle) -> Result<(), String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;

    std::fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("Failed to create app data dir: {}", e))?;

    // Initialize SQLite database for timetable caching
    let db_path = app_data_dir.join("timetable_cache.db");
    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // Create timetable cache table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS timetable_cache (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            route_pattern TEXT NOT NULL,
            line_id TEXT NOT NULL,
            from_id TEXT NOT NULL,
            to_id TEXT NOT NULL,
            dia_flg INTEGER NOT NULL,
            from_time TEXT NOT NULL,
            to_time TEXT,
            course_id TEXT,
            note TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create timetable_cache table: {}", e))?;

    // Create index for faster queries
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timetable_route
            ON timetable_cache(route_pattern, line_id, dia_flg)",
        [],
    )
    .map_err(|e| format!("Failed to create index: {}", e))?;

    // Create metadata table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache_metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )
    .map_err(|e| format!("Failed to create cache_metadata table: {}", e))?;

    // Store database path in app state
    if let Some(state) = app_handle.try_state::<AppState>() {
        if let Ok(mut db_path_lock) = state.db_path.lock() {
            *db_path_lock = Some(db_path);
        }
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            saved_stops: Mutex::new(Vec::new()),
            db_path: Mutex::new(None),
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
            init_database,
            clear_timetable_cache
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
