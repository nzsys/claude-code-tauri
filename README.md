# 札幌バストラッカー (Sapporo Bus Tracker)

札幌市の路線バスのリアルタイム情報を表示するクロスプラットフォームアプリケーションです。

## 機能

- 指定した時間帯に目的地を経由するバスの一覧表示
- バスの遅延状況の表示
- バスの混雑度の表示
- リアルタイムでのバス位置情報取得
- SQLiteによるデータの永続化

## 技術スタック

- **フロントエンド**: React + TypeScript + Vite
- **バックエンド**: Rust + Tauri
- **データベース**: SQLite
- **API**: 札幌市バスAPI (https://ekibus-api.city.sapporo.jp)

## セットアップ

### 必要な環境

- Node.js (v22以上)
- Rust (最新版)
- npm

### 依存関係のインストール

```bash
npm install
```

## 開発

### デスクトップ開発 (macOS/Windows/Linux)

```bash
npm run tauri dev
```

### プロダクションビルド

```bash
npm run tauri build
```

## モバイル対応

### Android

Android開発には以下が必要です：
- Android Studio
- Android SDK
- NDK

初期化:
```bash
npm run tauri android init
```

開発:
```bash
npm run tauri android dev
```

### iOS

iOS開発には以下が必要です（macOSのみ）：
- Xcode
- iOS SDK

初期化:
```bash
npm run tauri ios init
```

開発:
```bash
npm run tauri ios dev
```

## API仕様

アプリケーションは札幌市のえきバスナビAPIを使用しています：

- **バス停リスト取得**: `https://ekibus-api.city.sapporo.jp/Get_busstop_list`
- **バス接近情報取得**: `https://ekibus-api.city.sapporo.jp/Get_busstop_lastdata`

## データ永続化

アプリケーションはSQLiteを使用してデータを永続化します。データベースファイルはアプリケーションのデータディレクトリに保存されます。

## 推奨IDE設定

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## ライセンス

MIT License
