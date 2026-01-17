
`https://launcher.keychron.com/api/merchandise/product/vpId/875876392`:
```json
{
  "code": 200,
  "data": {
    "product": {
      "update_user": 1,
      "update_time": 1767766794000,
      "desc": null,
      "id": 546,
      "name": "Keychron Ultra-Link 8K",
      "category": 24,
      "pid": "0xD028",
      "vid": "0x3434",
      "vendor_product_id": "875876392",
      "cover": "https://sysmgr.keychron.cn/api/null",
      "dest": false,
      "support_update": 1,
      "support_bluetooth_update": null
    },
    "category": {
      "update_user": 1,
      "update_time": 1736828152000,
      "desc": "Keychron Link",
      "id": 24,
      "name": "接收器",
      "type": 2,
      "can_be_query": 1
    },
    "json": [],
    "firmware": {
      "count": 1,
      "lasted": {
        "update_user": 1,
        "update_time": 1767926539000,
        "desc": {
          "zh-CN": null,
          "en-US": null
        },
        "id": 1594,
        "deviceId": 546,
        "version": "0.1.6",
        "path": "https://sysmgr.keychron.cn/api/upload/bin/24/1762582390953.bin",
        "type": 0,
        "state": 1
      },
      "patch": null
    }
  },
  "message": "",
  "success": true
}
```

`https://launcher.keychron.com/api/merchandise/product/vpId/875876425`:
```json
{
  "code": 200,
  "data": {
    "product": {
      "update_user": 1,
      "update_time": 1760497282000,
      "desc": null,
      "id": 557,
      "name": "Keychron M6 8K",
      "category": 25,
      "pid": "0xD049",
      "vid": "0x3434",
      "vendor_product_id": "875876425",
      "cover": "upload/cover/25/1750133620332.png,upload/cover/25/1750133622319.png",
      "dest": true,
      "support_update": 1,
      "support_bluetooth_update": 0
    },
    "category": {
      "update_user": 1,
      "update_time": 1758600817000,
      "desc": null,
      "id": 25,
      "name": "M-鼠标",
      "type": 1,
      "can_be_query": 1
    },
    "json": [],
    "firmware": {
      "count": 1,
      "lasted": {
        "update_user": 7,
        "update_time": 1756799429000,
        "desc": {
          "zh-CN": "1.驱动增加profile 切换/ 回报率/dpi 档位数量设置等驱动新功能。",
          "en-US": "1. The driver adds new features such as profile switching/report rate/DPI gear number setting."
        },
        "id": 1176,
        "deviceId": 557,
        "version": "1.0.1",
        "path": "https://cdn.shopify.com/s/files/1/0059/0630/1017/files/release_keychron_m6_8k_fw1.0.1_2508041051.bin?v=1756364020",
        "type": 0,
        "state": 1
      },
      "patch": null
    }
  },
  "message": "",
  "success": true
}
```

`https://launcher.keychron.com/api/mouse/875876425.json`:
```json
{
  "name": "Keychron M6 8K",
  "type": "mouse",
  "keys": [
    {
      "x": 41.961414790996784,
      "y": 34.40514469453376,
      "dir": "left",
      "index": 0,
      "id": "left"
    },
    {
      "x": 64.14790996784566,
      "y": 17.041800643086816,
      "dir": "right",
      "id": "right",
      "index": 2
    },
    {
      "x": 57.226107226107224,
      "y": 24.592074592074592,
      "dir": "right",
      "id": "middle",
      "index": 1,
      "side": 1
    },
    {
      "x": 42.30769230769231,
      "y": 44.75524475524475,
      "dir": "left",
      "id": "forward",
      "index": 3,
      "side": 1
    },
    {
      "x": 44.40559440559441,
      "y": 58.04195804195804,
      "dir": "left",
      "id": "backward",
      "index": 4,
      "side": 1
    },
    {
      "x": 52.33100233100233,
      "y": 24.358974358974358,
      "dir": "left",
      "id": "leftTilt",
      "index": 9
    },
    {
      "x": 55.24044389642416,
      "y": 27.866831072749694,
      "dir": "right",
      "id": "rightTilt",
      "index": 8
    },
    {
      "x": 47.348951911220716,
      "y": 50.43156596794082,
      "dir": "right",
      "id": "rightScroll",
      "index": 10,
      "side": 1
    },
    {
      "x": 45.33799533799534,
      "y": 50.815850815850816,
      "dir": "left",
      "id": "leftScroll",
      "index": 11,
      "side": 1
    },
    {
      "x": 55.011655011655016,
      "y": 18.2983682983683,
      "dir": "up",
      "id": "upScroll",
      "index": 14
    },
    {
      "x": 55.361305361305355,
      "y": 30.76923076923077,
      "dir": "down",
      "id": "downScroll",
      "index": 13
    }
  ],
  "dpi": {
    "limit": [
      50,
      30000
    ],
    "level": [
      400,
      800,
      1600,
      3200,
      5000
    ],
    "maxReportRate": 8000,
    "reportRate": [
      {
        "value": 125,
        "color": "#fff"
      },
      {
        "value": 500,
        "color": "#0D99FF"
      },
      {
        "value": 1000,
        "color": "#F54242"
      },
      {
        "value": 2000,
        "color": [
          "#fff",
          "#0D99FF"
        ]
      },
      {
        "value": 4000,
        "color": [
          "#fff",
          "#F54242"
        ]
      },
      {
        "value": 8000,
        "color": [
          "#fff",
          "#0D99FF",
          "#F54242"
        ]
      }
    ]
  },
  "sys": {
    "lod": [
      {
        "index": 3,
        "value": "0.7mm",
        "label": "0.7mm"
      },
      {
        "index": 1,
        "value": "1.0mm",
        "label": "1.0mm"
      },
      {
        "index": 2,
        "value": "2.0mm",
        "label": "2.0mm"
      }
    ],
    "disSensor": false,
    "pairKeyModify": true
  },
  "light": false
}
```
