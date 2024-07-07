# wxbeacon2http

WxBeacon2の計測データをHTTPで公開します。

## 前提

### 動作確認OS

Windows / Linux

### 必要デバイス

- WxBeacon2(Omron社製 環境センサ 形2JCIE-BL01)
- BLE対応 Bluetooth 4.0 Adapter

### 機器設定

WxBeacon2のBeacon Modeを事前に`General Broadcaster 2(0x04)` 又は `Limited Broadcaster 2(0x05)`へ変更しておくこと。
ADV setting (Characteristics UUID: `0x3042`) の 8バイト目(mask: `0x0000000000000000FF00`)で設定可能。

詳細はOmronドキュメントを参照: https://omronfs.omron.com/ja_JP/ecb/products/pdf/CDSC-015.pdf

## API Reference

### status

#### req

```
GET http://{authority}/status
```

#### res

初回のセンサー値が取得できていれば

```json
"available"
```

まだなら

```json
"not_yet"
```

### sensor

#### req

```
GET http://{authority}/sensor
```

#### res

センサー値が取得できていれば

```json
{"timestamp":"2024-07-07T15:07:40.229555300Z","temperature":23.92,"humidity":70.08,"illuminance":115,"uv_index":0.02,"pressure":1001.2,"noise":36.72,"discomfort_index":72.24,"wgbt":22.91,"battery_voltage":2.9}
```

まだなら

`404 NotFound`