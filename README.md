# esp32-c3-examples

- 结合[wokwi](https://wokwi.com)的 esp32-c3 开发板在线练习
- wokwi 仿真外设在 esp32c3 中使用

## 开始之前准备

1. 在电脑中下载启动 wokwi 的命令 [wokwi-server](https://github.com/MabezDev/wokwi-server)

## 运行测试示例

[官方源码地址](https://github.com/esp-rs/esp-hal/tree/main/esp32c3-hal/examples)

## 外设 pac

- [led 灯](https://docs.wokwi.com/parts/wokwi-led)
  - `make run mode=led`
- [button 按钮](https://docs.wokwi.com/parts/wokwi-pushbutton)
  - `make run mode=button`
- [max7219 led 点阵屏](https://docs.wokwi.com/parts/wokwi-max7219-matrix)
  - `make run mode=max7219`
- [side-switch 开关](https://docs.wokwi.com/parts/wokwi-slide-switch)
  - `make run mode=switch`
- [ssd1306 oled 显示屏](https://docs.wokwi.com/parts/board-ssd1306)
  - `make run mode=ssd1306`
- [dht22 温湿传感器](https://docs.wokwi.com/zh-CN/parts/wokwi-dht22)
  - `make run mode=dht22`
- [pir-motion-sensor 被动红外（PIR）运动传感器](https://docs.wokwi.com/zh-CN/parts/wokwi-pir-motion-sensor)
  - `make run mode=pir`
