# guangzhou-metro

广州地铁简易导航

Rust语言与内存安全设计大作业

## 项目概述

这个项目实现了广州地铁站点之间的简易导航，站点数据收集自[广州地铁](https://www.gzmtr.com/tpgl/)，站点用时数据收集自[高德地图](https://m.amap.com/)

本项目以地铁站为节点，站点之间通勤所用时间为边构建图，使用Dijkstra算法求最短路径

## 项目依赖

- Rust 1.79.0
- Vite 5.2.0
- actix-web 4.0

## 运行教程

- 进入根目录，输入`cargo run`启动rust项目，rust后端服务器默认在8080端口启动
- 进入`front-end`目录，输入`npm install`安装基础依赖，之后输入`run dev`运行前端，前端默认在5173端口启动
- 网页访问`http://localhost:5173/`即可使用

## 结语

这个项目是鄙人Rust课程的大作业，在~~Gayhub~~上看到类似的项目，一拍脑袋便有了做这个项目的想法

上一次写前端还是在上次，前端水平属于幼教级别，边学边写了属于是，~~再写前端我就是狗~~

如果您发现了任何bug，可以提交issue或者发送[邮件](mailto:dreaminglri@outlook.com)


