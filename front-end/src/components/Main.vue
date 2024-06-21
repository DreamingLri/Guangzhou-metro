<script setup lang="ts">
import {onMounted, reactive, ref} from "vue"
import request from "../utills/request.ts";
import Subway from "./Subway.vue";

const line_list = ref()
const line = ref({})
const map_switch = ref(true)

interface Request{
  start: string,
  end: string,
}

const data: Request = reactive({
  start: '',
  end: ''
});

function getLine(){
  let message = {
    start: data.start[1],
    end: data.end[1]
  }
  request.post("getLine", message).then(res=>{
    line.value = res.data;
  })
}

interface Stations{
  label: string
  value: string
  children: any[]
}

function getStations(){
  request.get("getStation").then((res)=>{
    var stations = res.data
    var temps = []

    for(var line in stations){
      var line_temp: Stations = {
        value: line,
        label: line,
        children: []
      }
      for(var station_name in stations[line]){
        var station: Stations = {
          value: stations[line][station_name],
          label: stations[line][station_name],
          children: []
        }
        line_temp.children.push(station)
      }
      temps.push(line_temp)
    }
    line_list.value = temps
  })
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
          <el-aside width="60%" style="border-right: 1px solid #ccc">
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
              <a-image v-if="!map_switch" src="../../public/guangzhou metro.png"/>
              <Subway v-if="map_switch" style="height: auto"/>
            </div>
          </el-aside>
          <el-main>
            <div style="height: 100px">
              <div style="display: flex; justify-content: center">
                <el-cascader style="width: 100%; margin-right: 20px" v-model="data.start" :options="line_list" :props="props" placeholder="请选择或搜索起点站" filterable clearable/>
                <el-cascader style="width: 100%" v-model="data.end" :options="line_list" :props="props" placeholder="请选择或搜索终点站" filterable clearable/>
              </div>
              <div style="margin-top: 20px; display: flex; justify-content: center">
                <el-button style="width: 100%" @click="getLine" type="primary">开始导航</el-button>
              </div>
            </div>
            <div>

            </div>
          </el-main>
        </el-container>
      </el-scrollbar>

    </el-container>
  </div>
</template>

<style scoped>
</style>