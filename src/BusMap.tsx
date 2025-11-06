import { MapContainer, TileLayer, Marker, Popup } from "react-leaflet";
import L from "leaflet";
import "leaflet/dist/leaflet.css";

// Fix for default marker icons in webpack
import icon from "leaflet/dist/images/marker-icon.png";
import iconShadow from "leaflet/dist/images/marker-shadow.png";
import iconRetina from "leaflet/dist/images/marker-icon-2x.png";

// Set default icon
let DefaultIcon = L.icon({
  iconUrl: icon,
  iconRetinaUrl: iconRetina,
  shadowUrl: iconShadow,
  iconSize: [25, 41],
  iconAnchor: [12, 41],
  popupAnchor: [1, -34],
  shadowSize: [41, 41],
});

L.Marker.prototype.options.icon = DefaultIcon;

// Create custom bus icon
const busIcon = L.divIcon({
  html: `
    <div style="
      background: #3498db;
      color: white;
      border-radius: 50%;
      width: 30px;
      height: 30px;
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 18px;
      border: 3px solid white;
      box-shadow: 0 2px 8px rgba(0,0,0,0.3);
    ">
      🚌
    </div>
  `,
  className: "bus-marker",
  iconSize: [30, 30],
  iconAnchor: [15, 15],
  popupAnchor: [0, -15],
});

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
}

interface BusMapProps {
  buses: BusInfo[];
}

export default function BusMap({ buses }: BusMapProps) {
  // Filter buses with valid coordinates
  const busesWithLocation = buses.filter(
    (bus) => bus.latitude !== null && bus.longitude !== null
  );

  // Calculate center of map based on bus locations
  const center: [number, number] =
    busesWithLocation.length > 0
      ? [
          busesWithLocation.reduce((sum, bus) => sum + (bus.latitude || 0), 0) /
            busesWithLocation.length,
          busesWithLocation.reduce((sum, bus) => sum + (bus.longitude || 0), 0) /
            busesWithLocation.length,
        ]
      : [43.0642, 141.3469]; // Default to Sapporo

  function getDelayText(minutes: number | null): string {
    if (minutes === null) return "情報なし";
    if (minutes === 0) return "定刻";
    if (minutes > 0) return `${minutes}分遅れ`;
    return `${Math.abs(minutes)}分早い`;
  }

  return (
    <div className="map-container">
      <MapContainer
        center={center}
        zoom={13}
        style={{ height: "100%", width: "100%", borderRadius: "12px" }}
      >
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        {busesWithLocation.map((bus) => (
          <Marker
            key={bus.bus_id}
            position={[bus.latitude!, bus.longitude!]}
            icon={busIcon}
          >
            <Popup>
              <div style={{ minWidth: "200px" }}>
                <h3 style={{ margin: "0 0 10px 0" }}>{bus.route_name}</h3>
                <p style={{ margin: "5px 0" }}>
                  <strong>行き先:</strong> {bus.destination}
                </p>
                <p style={{ margin: "5px 0" }}>
                  <strong>バス停:</strong> {bus.stop_name}
                </p>
                {bus.arrival_time && (
                  <p style={{ margin: "5px 0" }}>
                    <strong>到着予定:</strong> {bus.arrival_time}
                  </p>
                )}
                <p style={{ margin: "5px 0" }}>
                  <strong>遅延:</strong> {getDelayText(bus.delay_minutes)}
                </p>
                {bus.congestion_level && (
                  <p style={{ margin: "5px 0" }}>
                    <strong>混雑度:</strong> {bus.congestion_level}
                  </p>
                )}
              </div>
            </Popup>
          </Marker>
        ))}
      </MapContainer>
      {busesWithLocation.length === 0 && (
        <div className="map-overlay">
          <p>位置情報のあるバスがありません</p>
        </div>
      )}
    </div>
  );
}
