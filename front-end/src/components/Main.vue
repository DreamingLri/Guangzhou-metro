<script setup lang="ts">
import {onMounted, reactive, ref} from "vue"
import request from "../utills/request.ts";

const stations = ref({})

interface Request{
  start: string,
  end: string,
}

const data: Request = reactive({
  start: '',
  end: ''
});

function getLine(){
  console.log(data)
  request.post("getLine", data).then(res=>{
    console.log(res)
  })
}

function getStations(){
  request.get("getStation").then((res)=>{
    stations.value = res
    console.log(stations.value)
  })
}

onMounted(()=>{
  getStations()
})
</script>

<template>
  <div class="common-layout">
    <el-container style="height: 100vh">
      <el-header height="100px">
        <div class="common-header">
          <h1 style="text-align: center">Guangzhou Metro</h1>
          <h4 style="text-align: center">广州地铁简易导航</h4>
        </div>
      </el-header>
      <el-scrollbar>
        <el-container>
          <el-aside width="60%" style="border-right: 1px solid #ccc">
            <div class="aside">
              <div style="height: 60px; text-align: center">
                <p>广州地铁线网示意图</p>
                <p>Guangzhou Metro Transport System Map</p>
              </div>
              <a-image
                  src="../../public/guangzhou metro.png"
              />
            </div>
          </el-aside>
          <el-main>
            <div>
              <el-input v-model="data.start" style="width: 240px" placeholder="Please input" />
              <el-input v-model="data.end" style="width: 240px" placeholder="Please input" />
              <el-button @click="getLine" type="primary">Primary</el-button>
            </div>
          </el-main>
        </el-container>
      </el-scrollbar>

    </el-container>
  </div>
</template>

<style scoped>
.common-layout{

}
</style>