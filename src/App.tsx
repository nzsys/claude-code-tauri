import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import DatePicker, { registerLocale } from "react-datepicker";
import { ja } from "date-fns/locale";
import "react-datepicker/dist/react-datepicker.css";
// import BusMap from "./BusMap";
import "./App.css";

registerLocale("ja", ja);

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
  latitude: number | null;
  longitude: number | null;
  stops_away: number | null;
  estimated_minutes: number | null;
  last_stop_name: string | null;
  course_id: string;
  end_station_id: string | null;
}

interface StationData {
  station_id: string;
  name: string;
}

interface BusStop {
  stop_id: string;
  stop_name: string;
  latitude: number | null;
  longitude: number | null;
}

interface StationPrediction {
  station_id: number;
  company_id: number;
  name: string;
}

function App() {
  // Bus search states
  const [departureStation, setDepartureStation] = useState("");
  const [arrivalStation, setArrivalStation] = useState("");
  const [selectedDateTime, setSelectedDateTime] = useState<Date>(new Date());
  const [useCurrentTime, setUseCurrentTime] = useState(true);
  const [buses, setBuses] = useState<BusInfo[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  // Saved bus stops
  const [savedStops, setSavedStops] = useState<BusStop[]>([]);

  // Station search states
  const [showDepartureSuggestions, setShowDepartureSuggestions] = useState(false);
  const [showArrivalSuggestions, setShowArrivalSuggestions] = useState(false);
  const [departureSuggestions, setDepartureSuggestions] = useState<StationPrediction[]>([]);
  const [arrivalSuggestions, setArrivalSuggestions] = useState<StationPrediction[]>([]);

  // Selected stations for display
  const [selectedDepartureStop, setSelectedDepartureStop] = useState<BusStop | null>(null);
  const [selectedArrivalStop, setSelectedArrivalStop] = useState<BusStop | null>(null);

  // Bus detail modal
  const [selectedBus, setSelectedBus] = useState<BusInfo | null>(null);
  const [showBusDetail, setShowBusDetail] = useState(false);
  const [busStops, setBusStops] = useState<StationData[]>([]);
  const [loadingStops, setLoadingStops] = useState(false);

  useEffect(() => {
    loadSavedStops();
  }, []);

  async function loadSavedStops() {
    try {
      const stops = await invoke<BusStop[]>("get_saved_bus_stops");
      setSavedStops(stops);
    } catch (err) {
      console.error("Failed to load saved stops:", err);
    }
  }

  async function saveBusStop(stop: BusStop) {
    try {
      const stops = await invoke<BusStop[]>("save_bus_stop", { stop });
      setSavedStops(stops);
    } catch (err) {
      console.error("Failed to save bus stop:", err);
    }
  }

  async function deleteBusStop(stopId: string) {
    try {
      const stops = await invoke<BusStop[]>("delete_bus_stop", { stopId });
      setSavedStops(stops);
    } catch (err) {
      console.error("Failed to delete bus stop:", err);
    }
  }

  async function searchStations(searchWord: string, isDeparture: boolean) {
    if (!searchWord || searchWord.length === 0) {
      if (isDeparture) {
        setDepartureSuggestions([]);
      } else {
        setArrivalSuggestions([]);
      }
      return;
    }

    try {
      const suggestions = await invoke<StationPrediction[]>(
        "search_station_suggestions",
        { searchWord }
      );
      if (isDeparture) {
        setDepartureSuggestions(suggestions);
      } else {
        setArrivalSuggestions(suggestions);
      }
    } catch (err) {
      console.error("駅検索に失敗しました:", err);
      if (isDeparture) {
        setDepartureSuggestions([]);
      } else {
        setArrivalSuggestions([]);
      }
    }
  }

  function selectStation(station: StationPrediction, isDeparture: boolean) {
    const busStop: BusStop = {
      stop_id: station.station_id.toString(),
      stop_name: station.name,
      latitude: null,
      longitude: null,
    };

    if (isDeparture) {
      setDepartureStation(station.name);
      setSelectedDepartureStop(busStop);
      setShowDepartureSuggestions(false);
      setDepartureSuggestions([]);
    } else {
      setArrivalStation(station.name);
      setSelectedArrivalStop(busStop);
      setShowArrivalSuggestions(false);
      setArrivalSuggestions([]);
    }
  }

  function selectSavedStop(stop: BusStop, isDeparture: boolean) {
    if (isDeparture) {
      setDepartureStation(stop.stop_name);
      setSelectedDepartureStop(stop);
      setShowDepartureSuggestions(false);
    } else {
      setArrivalStation(stop.stop_name);
      setSelectedArrivalStop(stop);
      setShowArrivalSuggestions(false);
    }
  }

  function swapStations() {
    const tempStation = departureStation;
    const tempStop = selectedDepartureStop;

    setDepartureStation(arrivalStation);
    setSelectedDepartureStop(selectedArrivalStop);

    setArrivalStation(tempStation);
    setSelectedArrivalStop(tempStop);
  }

  async function searchBuses(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);
    setError("");

    if (!selectedDepartureStop || !selectedArrivalStop) {
      setError("乗車駅と降車駅の両方を選択してください");
      setLoading(false);
      return;
    }

    try {
      // Save both stops
      await saveBusStop(selectedDepartureStop);
      await saveBusStop(selectedArrivalStop);

      const searchTime = useCurrentTime ? new Date() : selectedDateTime;
      const timeString = searchTime.toTimeString().slice(0, 5);

      // Call route search API with station IDs
      const result = await invoke<BusInfo[]>("search_buses", {
        request: {
          start_station_id: selectedDepartureStop.stop_id,
          end_station_id: selectedArrivalStop.stop_id,
          destination: arrivalStation,
          time_from: timeString,
          time_to: null,
        },
      });

      // Sort buses by arrival time (earliest first)
      const sortedBuses = result.sort((a, b) => {
        // Buses without arrival time go to the end
        if (!a.arrival_time && !b.arrival_time) return 0;
        if (!a.arrival_time) return 1;
        if (!b.arrival_time) return -1;

        // Compare arrival times
        return a.arrival_time.localeCompare(b.arrival_time);
      });

      setBuses(sortedBuses);

      // Reload saved stops
      await loadSavedStops();
    } catch (err) {
      setError(`検索に失敗しました: ${err}`);
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

  async function refreshTimetable() {
    try {
      await invoke("clear_timetable_cache");
      alert("時刻表キャッシュをクリアしました。次回の検索時に最新の時刻表を取得します。");
    } catch (err) {
      console.error("時刻表の更新に失敗しました:", err);
      alert(`時刻表の更新に失敗しました: ${err}`);
    }
  }

  async function openBusDetail(bus: BusInfo) {
    setSelectedBus(bus);
    setShowBusDetail(true);
    setBusStops([]);

    console.log("Bus details:", {
      course_id: bus.course_id,
      stop_id: bus.stop_id,
      end_station_id: bus.end_station_id,
    });

    // Only fetch stops if we have the required parameters
    if (bus.course_id && bus.end_station_id) {
      setLoadingStops(true);
      try {
        const stops = await invoke<StationData[]>("get_bus_stops_data", {
          courseId: bus.course_id,
          stationId: bus.stop_id,
          endSt: bus.end_station_id,
        });
        console.log("Received stops:", stops);
        setBusStops(stops);
      } catch (err) {
        console.error("Failed to load bus stops:", err);
      } finally {
        setLoadingStops(false);
      }
    } else {
      console.warn("Missing required parameters for fetching stops");
    }
  }

  function closeBusDetail() {
    setShowBusDetail(false);
  }

  return (
    <main className="container">
      <div className="header">
        <h1>札幌交通情報</h1>
        <button
          className="refresh-timetable-btn"
          onClick={refreshTimetable}
          title="時刻表キャッシュをクリアして最新版を取得します"
        >
          🔄 時刻表の更新
        </button>
      </div>

      {/* Search Section */}
      <div className="search-section">
        <form onSubmit={searchBuses}>
          {/* Departure Station */}
          <div className="form-group">
            <label htmlFor="departure">乗車駅:</label>
            <div className="autocomplete-wrapper">
              <input
                id="departure"
                type="text"
                value={departureStation}
                onChange={(e) => {
                  setDepartureStation(e.target.value);
                  searchStations(e.target.value, true);
                }}
                onFocus={() => setShowDepartureSuggestions(true)}
                placeholder="例: 大通、札幌駅"
              />
              {showDepartureSuggestions && (
                <div className="suggestions-dropdown">
                  <div className="suggestions-section">
                    <div className="suggestions-header">検索結果</div>
                    {departureSuggestions.map((station) => (
                      <div
                        key={`${station.station_id}-${station.company_id}`}
                        className="suggestion-item"
                        onClick={() => selectStation(station, true)}
                      >
                        {station.name}
                      </div>
                    ))}
                  </div>
                  {savedStops.length > 0 && (
                    <div className="suggestions-section">
                      <div className="suggestions-header">登録済みバス停</div>
                      {savedStops.map((stop) => (
                        <div
                          key={stop.stop_id}
                          className="suggestion-item saved"
                          onClick={() => selectSavedStop(stop, true)}
                        >
                          {stop.stop_name}
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              )}
            </div>
          </div>

          {/* Swap Button */}
          <div className="swap-button-container">
            <button
              type="button"
              className="swap-button"
              onClick={swapStations}
              title="乗車駅と降車駅を入れ替え"
            >
              ⇅
            </button>
          </div>

          {/* Arrival Station */}
          <div className="form-group">
            <label htmlFor="arrival">降車駅:</label>
            <div className="autocomplete-wrapper">
              <input
                id="arrival"
                type="text"
                value={arrivalStation}
                onChange={(e) => {
                  setArrivalStation(e.target.value);
                  searchStations(e.target.value, false);
                }}
                onFocus={() => setShowArrivalSuggestions(true)}
                placeholder="例: すすきの、真駒内"
              />
              {showArrivalSuggestions && (
                <div className="suggestions-dropdown">
                  <div className="suggestions-section">
                    <div className="suggestions-header">検索結果</div>
                    {arrivalSuggestions.map((station) => (
                      <div
                        key={`${station.station_id}-${station.company_id}`}
                        className="suggestion-item"
                        onClick={() => selectStation(station, false)}
                      >
                        {station.name}
                      </div>
                    ))}
                  </div>
                  {savedStops.length > 0 && (
                    <div className="suggestions-section">
                      <div className="suggestions-header">登録済みバス停</div>
                      {savedStops.map((stop) => (
                        <div
                          key={stop.stop_id}
                          className="suggestion-item saved"
                          onClick={() => selectSavedStop(stop, false)}
                        >
                          {stop.stop_name}
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              )}
            </div>
          </div>

          {/* Time Selection */}
          <div className="form-group">
            <label htmlFor="time-mode">出発時刻:</label>
            <div className="toggle-switch-container">
              <span className={`toggle-label ${useCurrentTime ? 'active' : ''}`}>現在時刻</span>
              <div className="toggle-switch" onClick={() => setUseCurrentTime(!useCurrentTime)}>
                <input
                  type="checkbox"
                  checked={!useCurrentTime}
                  onChange={() => setUseCurrentTime(!useCurrentTime)}
                  className="toggle-input"
                />
                <span className="toggle-slider"></span>
              </div>
              <span className={`toggle-label ${!useCurrentTime ? 'active' : ''}`}>指定日時</span>
            </div>
          </div>

          {!useCurrentTime && (
            <div className="form-group">
              <label htmlFor="datetime">日時を指定:</label>
              <DatePicker
                selected={selectedDateTime}
                onChange={(date) => date && setSelectedDateTime(date)}
                showTimeSelect
                timeFormat="HH:mm"
                timeIntervals={15}
                dateFormat="yyyy年MM月dd日 HH:mm"
                className="datetime-picker"
                locale="ja"
              />
            </div>
          )}

          <div className="button-group">
            <button type="submit" disabled={loading}>
              {loading ? "検索中..." : "バス検索"}
            </button>
          </div>
        </form>

        {error && <div className="error">{error}</div>}
      </div>

      {/* Results */}
      {buses.length > 0 && (
        <div className="results-section">
          <h2>検索結果 ({buses.length}件)</h2>
          <div className="results-layout">
            {/* Bus/Transit List */}
            <div className="bus-list-container">
              <div className="bus-list">
                {buses.map((bus, index) => (
                  <div
                    key={`${bus.bus_id}-${index}`}
                    className="bus-card clickable"
                    onClick={() => openBusDetail(bus)}
                  >
                    <div className="bus-header">
                      <div className="bus-title">
                        <h3>{bus.route_name}</h3>
                        <span className="destination">{bus.destination}行き</span>
                      </div>
                      <button
                        className="refresh-bus-btn"
                        onClick={(e) => {
                          e.stopPropagation();
                          // TODO: バス情報を更新する関数を実装
                          alert("バス情報の更新機能は実装予定です");
                        }}
                        title="最新情報に更新"
                      >
                        🔄
                      </button>
                    </div>

                    {/* Current Status Highlight */}
                    {(bus.last_stop_name || bus.stops_away !== null) && (
                      <div className="bus-current-status">
                        {bus.last_stop_name && (
                          <div className="status-item">
                            <span className="status-icon">📍</span>
                            <span className="status-text">現在地: {bus.last_stop_name}</span>
                          </div>
                        )}
                        {bus.stops_away !== null && (
                          <div className="status-item">
                            <span className="status-icon">🚌</span>
                            <span className="status-text">あと{bus.stops_away}駅</span>
                          </div>
                        )}
                      </div>
                    )}

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
                      {bus.estimated_minutes !== null && (
                        <div className="detail-item">
                          <span className="label">到着まで:</span>
                          <span className="estimated-time">約{bus.estimated_minutes}分</span>
                        </div>
                      )}
                    </div>
                    <div className="bus-footer">
                      <small>更新: {new Date(bus.updated_at).toLocaleString("ja-JP")}</small>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* Map - Temporarily disabled */}
            {/* <div className="map-section">
              <h3>バス現在地マップ</h3>
              <BusMap buses={buses} />
            </div> */}
          </div>
        </div>
      )}

      {/* Saved Bus Stops */}
      <div className="bus-stops-section">
        <h2>登録済みバス停</h2>
        {savedStops.length === 0 ? (
          <p className="no-results">登録済みバス停はありません。検索して駅を選択すると自動的に登録されます。</p>
        ) : (
          <div className="bus-stops-list">
            {savedStops.map((stop) => (
              <div key={stop.stop_id} className="bus-stop-item">
                <span className="stop-name">{stop.stop_name}</span>
                <button
                  className="delete-button"
                  onClick={() => deleteBusStop(stop.stop_id)}
                  title="削除"
                >
                  ✕
                </button>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Bus Detail Modal */}
      {showBusDetail && selectedBus && (
        <div className="modal-overlay" onClick={closeBusDetail}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>{selectedBus.route_name}</h2>
              <button className="modal-close" onClick={closeBusDetail}>✕</button>
            </div>
            <div className="modal-body">
              <div className="bus-detail-info">
                <p><strong>行先:</strong> {selectedBus.destination}</p>
                <p><strong>到着予定:</strong> {selectedBus.arrival_time || "情報なし"}</p>
                <p><strong>遅延:</strong> {getDelayText(selectedBus.delay_minutes)}</p>
                <p><strong>混雑度:</strong> {selectedBus.congestion_level || "不明"}</p>
                {selectedBus.last_stop_name && (
                  <p><strong>最終通過停留所:</strong> {selectedBus.last_stop_name}</p>
                )}
                {selectedBus.stops_away !== null && (
                  <p><strong>残り停留所数:</strong> あと{selectedBus.stops_away}駅</p>
                )}
              </div>
              <div className="bus-stops-timeline">
                <h3>停留所一覧</h3>
                {loadingStops ? (
                  <p className="info-text">読み込み中...</p>
                ) : busStops.length > 0 ? (
                  <div className="stops-list">
                    {busStops.map((stop) => (
                      <div
                        key={stop.station_id}
                        className={`stop-item ${
                          stop.station_id === selectedBus.stop_id ? "current-stop" : ""
                        }`}
                      >
                        <div className="stop-marker">
                          {stop.station_id === selectedBus.stop_id ? "●" : "○"}
                        </div>
                        <div className="stop-info">
                          <div className="stop-name">{stop.name}</div>
                          {stop.station_id === selectedBus.stop_id && (
                            <div className="stop-label">現在地</div>
                          )}
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <p className="info-text">停留所情報が取得できませんでした</p>
                )}
              </div>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;
