import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

// Sapporo Transit API types
interface RouteListItem {
  line_id: number;
  line_name: string;
  company_id: string;
  from_id: number;
  from_name: string;
  to_id: number;
  to_name: string;
  sec_time: number;
  fare: string;
  c_fare: number;
  line_type: number;
  connect_time: number;
  connect_distance: number;
  connect_flag: number;
  service_count: number;
}

interface WalkData {
  start_walk_time: number;
  start_walk_dist: number;
  goal_walk_time: number;
  goal_walk_dist: number;
}

interface Route {
  total_time: number;
  total_connect_time: number;
  line_count: number;
  fare: number;
  c_fare: number;
  route_list: RouteListItem[];
  pattern: string;
  service_count: number;
  service_count_raw: number;
  service_frequency: number;
  walk_data: WalkData;
}

interface SearchRouteResponse {
  result: string;
  search_route: {
    route: Route[];
  };
}

interface TimeTableEntry {
  from_time: string;
  to_time: string;
  note: string;
  fromto: string;
  course_id: number;
  connect_index: number;
  prev_index: number;
}

interface DiaList {
  dia_flg: number;
  time_table: TimeTableEntry[];
}

interface TimetableCourse {
  course_id: number;
  course_name: string;
  line_id: number;
  line_name: string;
}

interface RouteListForTimetable {
  line_id: string;
  from_id: string;
  to_id: string;
  connect_time: number;
  connect_distance: number;
  line_name: string;
  from_name: string;
  to_name: string;
  course_list: TimetableCourse[];
  dia_list: DiaList[];
}

interface SearchRouteTimetableResponse {
  result: string;
  search_route_timetable: {
    time_table: {
      from_id: string;
      to_id: string;
      pattern: string;
      from_name: string;
      to_name: string;
      day_type: number;
      route_list: RouteListForTimetable[];
    };
  };
}

interface BusStop {
  stop_id: string;
  stop_name: string;
  latitude: number | null;
  longitude: number | null;
}

function App() {
  const [startStation, setStartStation] = useState("420005"); // 南６条西１１丁目
  const [endStation, setEndStation] = useState("420004"); // すすきの
  const [routes, setRoutes] = useState<Route[]>([]);
  const [selectedRoute, setSelectedRoute] = useState<Route | null>(null);
  const [timetable, setTimetable] = useState<SearchRouteTimetableResponse["search_route_timetable"]["time_table"] | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  async function searchRoutes(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);
    setError("");
    setRoutes([]);
    setSelectedRoute(null);
    setTimetable(null);

    try {
      const result = await invoke<SearchRouteResponse>("search_routes", {
        startSt: startStation,
        endSt: endStation,
      });

      if (result.result === "0" && result.search_route.route.length > 0) {
        setRoutes(result.search_route.route);
      } else {
        setError("ルートが見つかりませんでした");
      }
    } catch (err) {
      setError(`検索に失敗しました: ${err}`);
      console.error("Search error:", err);
    } finally {
      setLoading(false);
    }
  }

  async function fetchRouteTimetable(route: Route) {
    setLoading(true);
    setError("");
    setSelectedRoute(route);

    try {
      const result = await invoke<SearchRouteTimetableResponse>(
        "get_route_timetable",
        {
          pattern: route.pattern,
        }
      );

      if (result.result === "0") {
        setTimetable(result.search_route_timetable.time_table);
      } else {
        setError("時刻表の取得に失敗しました");
      }
    } catch (err) {
      setError(`時刻表の取得に失敗しました: ${err}`);
      console.error("Timetable error:", err);
    } finally {
      setLoading(false);
    }
  }

  function formatTime(minutes: number): string {
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    return hours > 0 ? `${hours}時間${mins}分` : `${mins}分`;
  }

  function getDayTypeName(diaFlg: number): string {
    switch (diaFlg) {
      case 1:
        return "平日";
      case 2:
        return "土曜日";
      case 3:
        return "日曜・祝日";
      default:
        return `ダイヤ ${diaFlg}`;
    }
  }

  return (
    <main className="container">
      <h1>札幌バス・地下鉄ルート検索</h1>

      <div className="search-section">
        <h2>ルート検索</h2>
        <form onSubmit={searchRoutes}>
          <div className="form-group">
            <label htmlFor="start-station">出発駅ID:</label>
            <input
              id="start-station"
              type="text"
              value={startStation}
              onChange={(e) => setStartStation(e.target.value)}
              placeholder="例: 420005 (南６条西１１丁目)"
            />
          </div>

          <div className="form-group">
            <label htmlFor="end-station">到着駅ID:</label>
            <input
              id="end-station"
              type="text"
              value={endStation}
              onChange={(e) => setEndStation(e.target.value)}
              placeholder="例: 420004 (すすきの)"
            />
          </div>

          <div className="button-group">
            <button type="submit" disabled={loading}>
              {loading ? "検索中..." : "ルート検索"}
            </button>
          </div>
        </form>

        {error && <div className="error">{error}</div>}
      </div>

      {routes.length > 0 && (
        <div className="results-section">
          <h2>検索結果 ({routes.length}件のルート)</h2>
          <div className="bus-list">
            {routes.map((route, index) => (
              <div
                key={index}
                className="bus-card"
                onClick={() => fetchRouteTimetable(route)}
                style={{ cursor: "pointer" }}
              >
                <div className="bus-header">
                  <h3>ルート {index + 1}</h3>
                  <span className="destination">
                    所要時間: {formatTime(route.total_time)}
                  </span>
                </div>
                <div className="bus-details">
                  <div className="detail-item">
                    <span className="label">運賃:</span>
                    <span>¥{route.fare} (子供: ¥{route.c_fare})</span>
                  </div>
                  <div className="detail-item">
                    <span className="label">乗換回数:</span>
                    <span>{route.line_count - 1}回</span>
                  </div>
                  <div className="detail-item">
                    <span className="label">運行本数:</span>
                    <span>{route.service_count}本/日</span>
                  </div>
                </div>
                <div className="route-details">
                  {route.route_list.map((segment, segIndex) => (
                    <div key={segIndex} className="route-segment">
                      <strong>{segment.line_name}</strong>
                      <br />
                      {segment.from_name} → {segment.to_name} ({segment.sec_time}分)
                    </div>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {timetable && selectedRoute && (
        <div className="timetable-section">
          <h2>
            時刻表: {timetable.from_name} → {timetable.to_name}
          </h2>
          {timetable.route_list.map((routeItem, index) => (
            <div key={index} className="timetable-route">
              <h3>{routeItem.line_name}</h3>
              {routeItem.dia_list.map((dia, diaIndex) => (
                <div key={diaIndex} className="dia-section">
                  <h4>{getDayTypeName(dia.dia_flg)}</h4>
                  <div className="time-table-grid">
                    {dia.time_table.map((entry, entryIndex) => (
                      <div key={entryIndex} className="time-entry">
                        {entry.from_time} → {entry.to_time}
                      </div>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          ))}
        </div>
      )}

      <div className="help-section">
        <h3>駅IDの例:</h3>
        <ul>
          <li>420005: 南６条西１１丁目</li>
          <li>420004: すすきの</li>
          <li>420001: 札幌駅前</li>
        </ul>
      </div>
    </main>
  );
}

export default App;
