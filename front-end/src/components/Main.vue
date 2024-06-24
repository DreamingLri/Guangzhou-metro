<script setup lang="ts">
import {onMounted, reactive, ref} from "vue"
import request from "../utills/request.ts";
import Subway from "./Subway.vue";
import {ElMessage} from "element-plus";

const line_list = ref()
const lines = ref()
const map_switch = ref(true)

interface Request{
  start: string | null,
  end: string | null,
}

const data: Request = reactive({
  start: '',
  end: ''
});

function getLine(){
  let message: Request = {
    start: '',
    end: '',
  }
  if(localStorage.getItem("start") && localStorage.getItem("end")){
    message.start = localStorage.getItem("start")
    message.end = localStorage.getItem("end")
    localStorage.clear()
    console.log(message)
  } else {
    if(data.start && data.end){
      message.start = data.start[1]
      message.end = data.end[1]
    }
  }
  // console.log(localStorage.getItem("start") + " " + localStorage.getItem("end"))

  if (JSON.stringify(message) === '{}' || !message.start || !message.end) {
    ElMessage.warning("请选择起点站和终点站")
  } else {
    request.post("getLine", message).then(res=>{
      lines.value = res.data;
    })
  }
}

interface Stations{
  label: string
  value: string
  children: any[]
}

function getStations(){
  request.get("getStation").then((res)=>{
    let stations = res.data
    let temps = []

    for(let line in stations){
      let line_temp: Stations = {
        value: line,
        label: line,
        children: []
      }
      for(let station_name in stations[line]){
        let station: Stations = {
          value: stations[line][station_name],
          label: stations[line][station_name],
          children: []
        }
        line_temp.children.push(station)
      }
      temps.push(line_temp)
    }
    temps = temps.filter((item) => item.value !== '换乘')
    line_list.value = temps
  })
}

function toHourMinute(minutes: number){
  if(Math.floor(minutes/60) === 0){
    return (minutes % 60)+ "分钟"
  } else {
    return (Math.floor(minutes/60) + "小时" + (minutes%60) + "分钟" );
  }
}

const color_table = {
  "1号线": "rgb(246,215,72)",
  "2号线": "rgb(42,99,173)",
  "3号线": "rgb(242,169,59)",
  "3号线北延线": "rgb(242,169,59)",
  "4号线": "rgb(56,128,64)",
  "5号线": "rgb(181,37,65)",
  "6号线": "rgb(107,36,79)",
  "7号线": "rgb(145,183,63)",
  "8号线": "rgb(56,127,133)",
  "9号线": "rgb(122,187,148)",
  "13号线": "rgb(130,133,62)",
  "14号线": "rgb(124,38,45)",
  "14号线知识城支线": "rgb(124,38,45)",
  "18号线": "rgb(44,72,157)",
  "21号线": "rgb(13,19,51)",
  "22号线": "rgb(167,95,46)",
  "APM线": "rgb(82,179,222)",
  "佛山2号线": "rgb(231,52,35)",
  "佛山3号线": "rgb(42,99,173)",
  "南海有轨1号线": "rgb(72,160,220)",
  "广佛线": "rgb(189,209,66)",
  "海珠有轨1号线": "#5eb630",
  "黄埔有轨1号线": "#bd0000",
  "换乘": "rgb(153,160,169)"
}


const props = {
  expandTrigger: 'hover' as const,
}

onMounted(()=>{
  getStations()
})
</script>

<template>
  <div class="common-layout">
    <el-container style="height: 100vh">
      <el-header height="100px">
        <div class="common-header" style="margin-top: 20px">
          <h1 style="text-align: center">Guangzhou Metro</h1>
          <h4 style="text-align: center">广州地铁简易导航</h4>
        </div>
      </el-header>
      <el-scrollbar>
        <el-container>
          <el-aside width="65%" style="border-right: 1px solid #ccc">
            <div class="aside">
              <div style="height: 80px; text-align: center; display: flex; align-items: center;">
                <div style="width: 125px"/>
                <div style="margin: 0 auto">
                  <p>广州地铁线网示意图</p>
                  <p>Guangzhou Metro Transport System Map</p>
                </div>
                <div style="height: 100px; width: 125px; display: flex; align-items: center;">
                  <div style="height: 100px">
                    <p style="line-height: 100px; margin-right: 10px">地图切换</p>
                  </div>
                  <el-switch style="margin-right: 10px" v-model="map_switch" />
                </div>
              </div>
<!--              <a-image v-if="!map_switch" src="../../public/guangzhou metro.png"/>-->
              <Subway style="height: auto" :map_switch="map_switch"/>
            </div>
          </el-aside>
          <el-main>
            <div style="height: 100px">
              <div style="display: flex; justify-content: center">
                <el-cascader style="width: 100%; margin-right: 20px" v-model="data.start" :options="line_list" :props="props" placeholder="请选择或搜索起点站" filterable clearable :show-all-levels="false"/>
                <el-cascader style="width: 100%" v-model="data.end" :options="line_list" :props="props" placeholder="请选择或搜索终点站" filterable clearable :show-all-levels="false"/>
              </div>
              <div style="margin-top: 20px; display: flex; justify-content: center">
                <el-button style="width: 100%" @click="getLine" type="primary">开始导航</el-button>
              </div>
            </div>
            <div>
              <div v-if="lines || false">
                <el-card shadow="hover">
                  <div>
                    全程 {{toHourMinute(lines.len)}}
                  </div>
                  <div style="margin-top: 10px">
                    <el-button
                        v-for="line in lines.segments"
                        :color="color_table[line.line as keyof typeof color_table]"
                        style="color: white; margin-top: 10px;"
                        round
                    >
                      {{line.line !== '换乘' ? line.line : '步行出站换乘'}}
                    </el-button>
                  </div>
                </el-card>
                <el-card style="margin-top: 10px" shadow="hover" v-for="line in lines.segments">
                  <div style="display: flex">
                    <div style="width: 10px; height: auto ; border-radius: 5px; margin-right: 8px;" :style="{'background-color': color_table[line.line as keyof typeof color_table]}"/>
                    <div style="width: 100%">
                      <div style="margin-bottom: 5px">
                        <div style="display: flex">
                          <div>
                            <b>{{line.stations[0]}} 地铁站</b>
                          </div>
                          <div style="height: 24px">
                            <a style="font-size: 14px; line-height: 24px; margin-left: 5px">用时约 {{line.len}} 分钟</a>
                          </div>
                        </div>
                        <div style="height: 24px; display: flex; align-items: center; margin-top: 5px">
                          <el-button :color="color_table[line.line as keyof typeof color_table] " style="color: white" size="small">
                            <a style="font-size: 14px">{{line.line}}</a>
                          </el-button>
                          <div style="height: 24px">
                            <a v-if="line.line !== '换乘'" style="font-size: 14px; line-height: 24px; margin-left: 5px">{{line.direction}}方向</a>
                          </div>
                        </div>
                      </div>
                      <p v-for="station in line.stations.slice(1, line.stations.length - 1)" style="height: 7px; font-size: 14px">
                        <b>{{station}}</b>
                      </p>
                      <div>
                        <b>{{line.stations[line.stations.length - 1]}} 地铁站</b>
                      </div>
                    </div>
                  </div>
                </el-card>
              </div>
            </div>
          </el-main>
        </el-container>
      </el-scrollbar>

    </el-container>
  </div>
</template>

<style scoped>
</style>