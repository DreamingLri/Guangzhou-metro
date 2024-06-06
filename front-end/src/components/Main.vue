<script setup lang="ts">

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
          <el-main>Main</el-main>
        </el-container>
      </el-scrollbar>

    </el-container>
  </div>
</template>

<style scoped>
.common-layout{

}
</style>