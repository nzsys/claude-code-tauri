import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface BusInfo {
  bus_id: string;
  route_name: string;
  destination: string;
  stop_id: string;
  stop_name: string;
  arrival_time: string | null;
  delay_minutes: number | null;
  congestion_level: string | null;
  updated_at: string;
}

interface BusStop {
  stop_id: string;
  stop_name: string;
  latitude: number | null;
  longitude: number | null;
}

function App() {
  const [destination, setDestination] = useState("");
  const [timeFrom, setTimeFrom] = useState("");
  const [timeTo, setTimeTo] = useState("");
  const [buses, setBuses] = useState<BusInfo[]>([]);
  const [busStops, setBusStops] = useState<BusStop[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  useEffect(() => {
    loadBusStops();
  }, []);

  async function loadBusStops() {
    try {
      const stops = await invoke<BusStop[]>("get_bus_stops");
      setBusStops(stops);
    } catch (err) {
      console.error("Failed to load bus stops:", err);
    }
  }

  async function searchBuses(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);
    setError("");

    try {
      const result = await invoke<BusInfo[]>("search_buses", {
        request: {
          destination: destination || null,
          time_from: timeFrom || null,
          time_to: timeTo || null,
        },
      });
      setBuses(result);
    } catch (err) {
      setError(`検索に失敗しました: ${err}`);
    } finally {
      setLoading(false);
    }
  }

  async function fetchLiveData() {
    setLoading(true);
    setError("");

    try {
      const stopIds = busStops.map((stop) => stop.stop_id);
      const data = await invoke<string>("fetch_bus_location_data", {
        stopIdList: stopIds,
      });
      console.log("Bus location data:", data);
      // TODO: Parse and display the data
    } catch (err) {
      setError(`データ取得に失敗しました: ${err}`);
    } finally {
      setLoading(false);
    }
  }

  function getCongestionColor(level: string | null): string {
    if (!level) return "gray";
    switch (level) {
      case "低":
        return "green";
      case "中":
        return "orange";
      case "高":
        return "red";
      default:
        return "gray";
    }
  }

  function getDelayText(minutes: number | null): string {
    if (minutes === null) return "情報なし";
    if (minutes === 0) return "定刻";
    if (minutes > 0) return `${minutes}分遅れ`;
    return `${Math.abs(minutes)}分早い`;
  }

  return (
    <main className="container">
      <h1>札幌バストラッカー</h1>

      <div className="search-section">
        <h2>バス検索</h2>
        <form onSubmit={searchBuses}>
          <div className="form-group">
            <label htmlFor="destination">目的地:</label>
            <input
              id="destination"
              type="text"
              value={destination}
              onChange={(e) => setDestination(e.target.value)}
              placeholder="例: 札幌駅"
            />
          </div>

          <div className="form-group">
            <label htmlFor="time-from">開始時刻:</label>
            <input
              id="time-from"
              type="time"
              value={timeFrom}
              onChange={(e) => setTimeFrom(e.target.value)}
            />
          </div>

          <div className="form-group">
            <label htmlFor="time-to">終了時刻:</label>
            <input
              id="time-to"
              type="time"
              value={timeTo}
              onChange={(e) => setTimeTo(e.target.value)}
            />
          </div>

          <div className="button-group">
            <button type="submit" disabled={loading}>
              {loading ? "検索中..." : "検索"}
            </button>
            <button type="button" onClick={fetchLiveData} disabled={loading}>
              リアルタイムデータ取得
            </button>
          </div>
        </form>

        {error && <div className="error">{error}</div>}
      </div>

      <div className="results-section">
        <h2>バス一覧</h2>
        {buses.length === 0 ? (
          <p className="no-results">検索結果がありません</p>
        ) : (
          <div className="bus-list">
            {buses.map((bus) => (
              <div key={bus.bus_id} className="bus-card">
                <div className="bus-header">
                  <h3>{bus.route_name}</h3>
                  <span className="destination">{bus.destination}行き</span>
                </div>
                <div className="bus-details">
                  <div className="detail-item">
                    <span className="label">バス停:</span>
                    <span>{bus.stop_name}</span>
                  </div>
                  <div className="detail-item">
                    <span className="label">到着予定:</span>
                    <span>{bus.arrival_time || "情報なし"}</span>
                  </div>
                  <div className="detail-item">
                    <span className="label">遅延状況:</span>
                    <span className="delay-info">
                      {getDelayText(bus.delay_minutes)}
                    </span>
                  </div>
                  <div className="detail-item">
                    <span className="label">混雑度:</span>
                    <span
                      className="congestion-badge"
                      style={{
                        backgroundColor: getCongestionColor(
                          bus.congestion_level
                        ),
                      }}
                    >
                      {bus.congestion_level || "不明"}
                    </span>
                  </div>
                </div>
                <div className="bus-footer">
                  <small>更新: {new Date(bus.updated_at).toLocaleString("ja-JP")}</small>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      <div className="bus-stops-section">
        <h2>登録済みバス停</h2>
        <div className="bus-stops-list">
          {busStops.map((stop) => (
            <div key={stop.stop_id} className="bus-stop-item">
              <span className="stop-name">{stop.stop_name}</span>
              <span className="stop-id">ID: {stop.stop_id}</span>
            </div>
          ))}
        </div>
      </div>
    </main>
  );
}

export default App;
