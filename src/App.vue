<template>
  <main class="container">

    <h1>全盘搜自动签到工具</h1>

    <h3>配置项</h3>
    <section class="box">
      <div class="input-group">
        <input required="" type="text" name="text" autocomplete="off" class="input" v-model="username">
        <label class="user-label">用户名</label>
      </div>

      <div class="input-group pwd">
        <input required="" type="password" name="text" autocomplete="off" class="input" v-model="password">
        <label class="user-label">密码</label>
      </div>

      <div class="switch-base">
        <span>开机自启</span>
        <label class="plane-switch">
          <input type="checkbox" v-model="autoStart">
          <div>
            <div>
              <svg viewBox="0 0 13 13">
                <path
                  d="M1.55989957,5.41666667 L5.51582215,5.41666667 L4.47015462,0.108333333 L4.47015462,0.108333333 C4.47015462,0.0634601974 4.49708054,0.0249592654 4.5354546,0.00851337035 L4.57707145,0 L5.36229752,0 C5.43359776,0 5.50087375,0.028779451 5.55026392,0.0782711996 L5.59317877,0.134368264 L7.13659662,2.81558333 L8.29565964,2.81666667 C8.53185377,2.81666667 8.72332694,3.01067661 8.72332694,3.25 C8.72332694,3.48932339 8.53185377,3.68333333 8.29565964,3.68333333 L7.63589819,3.68225 L8.63450135,5.41666667 L11.9308317,5.41666667 C12.5213171,5.41666667 13,5.90169152 13,6.5 C13,7.09830848 12.5213171,7.58333333 11.9308317,7.58333333 L8.63450135,7.58333333 L7.63589819,9.31666667 L8.29565964,9.31666667 C8.53185377,9.31666667 8.72332694,9.51067661 8.72332694,9.75 C8.72332694,9.98932339 8.53185377,10.1833333 8.29565964,10.1833333 L7.13659662,10.1833333 L5.59317877,12.8656317 C5.55725264,12.9280353 5.49882018,12.9724157 5.43174295,12.9907056 L5.36229752,13 L4.57707145,13 L4.55610333,12.9978962 C4.51267695,12.9890959 4.48069792,12.9547924 4.47230803,12.9134397 L4.47223088,12.8704208 L5.51582215,7.58333333 L1.55989957,7.58333333 L0.891288881,8.55114605 C0.853775374,8.60544678 0.798421006,8.64327676 0.73629202,8.65879796 L0.672314689,8.66666667 L0.106844414,8.66666667 L0.0715243949,8.66058466 L0.0715243949,8.66058466 C0.0297243066,8.6457608 0.00275502199,8.60729104 0,8.5651586 L0.00593007386,8.52254537 L0.580855011,6.85813984 C0.64492547,6.67265611 0.6577034,6.47392717 0.619193545,6.28316421 L0.580694768,6.14191703 L0.00601851064,4.48064746 C0.00203480725,4.4691314 0,4.45701613 0,4.44481314 C0,4.39994001 0.0269259152,4.36143908 0.0652999725,4.34499318 L0.106916826,4.33647981 L0.672546853,4.33647981 C0.737865848,4.33647981 0.80011301,4.36066329 0.848265401,4.40322477 L0.89131128,4.45169723 L1.55989957,5.41666667 Z"
                  fill="currentColor"></path>
              </svg>
            </div>
            <span class="street-middle"></span>
            <span class="cloud"></span>
            <span class="cloud two"></span>
          </div>
        </label>
      </div>


      <div class="save-box">
        <button class="button" @click="handleSaveAction">
          <span class="lable">保存</span>
        </button>
      </div>
    </section>


    <p class="tip" v-if="tipInfo">{{ tipInfo }}</p>


  </main>
</template>


<script setup>

/** 
import { invoke } from "@tauri-apps/api/core";
async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
*/


import { ref, onMounted, watch } from "vue";
import http, { token } from "@/utils/http.js"
import { saveValue, getValue } from "@/utils/store.js"
import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart'

import { createTray } from "@/utils/tray.js"
import { listen } from '@tauri-apps/api/event';






const username = ref("")
const password = ref("")
const autoStart = ref(false)

onMounted(async () => {
  try {
    const { value: { name, pwd } } = await getValue()
    if (name) {
      username.value = name
    }
    if (pwd) {
      password.value = pwd
    }

    startReq()

    setInterval(() => {
      startReq()
    }, 8 * 60 * 60 * 1000);


  } catch (error) {
    console.log('error', error)
  }

  const auto = await isEnabled()
  autoStart.value = auto

  createTray()

  listen('didClickClose', (event) => {
    console.log("点击关闭", event)
  });

})






const tipInfo = ref()
const handleSaveAction = async () => {

  saveValue({ value: { name: username.value, pwd: password.value } })
  if (autoStart.value) {
    await enable()
  } else {
    disable()
  }
  startReq()
}

const startReq = async () => {
  tipInfo.value = ""


  const res0 = await http("/v1/user/login", { username: username.value, password: password.value })
  // console.log('res0', res0)

  if (res0.code == 200) {
    const resToken = res0.data.token

    if (resToken) {
      token.value = resToken
      const res1 = await http("/auth/v1/user/check_in?token=", {}, "GET")
      // console.log('res1', res1)

      if (res1.code == 200) {
        const { coin, continue_days } = res1.data
        tipInfo.value = `已获得${coin}积分，连续签到${continue_days}天`
      } else {
        tipInfo.value = res0.msg
      }
    }
  } else {
    tipInfo.value = res0.msg
  }
}




</script>


<style scoped>
.tip {
  color: red;
}


.container {
  padding: 20px;
}

.box {
  display: flex;
  flex-direction: column;

}



.input-group {
  position: relative;

}

.input-group.pwd {
  margin-top: 20px;
}


.input {
  border: solid 1.5px #9e9e9e;
  border-radius: 1rem;
  background: none;
  padding: 1rem;
  font-size: 1rem;
  transition: border 150ms cubic-bezier(0.4, 0, 0.2, 1);
}

.user-label {
  position: absolute;
  left: 15px;
  pointer-events: none;
  transform: translateY(1rem);
  transition: 150ms cubic-bezier(0.4, 0, 0.2, 1);
}

.input:focus,
input:valid {
  outline: none;
  border: 1.5px solid #1a73e8;
}

.input:focus~label,
input:valid~label {
  transform: translateY(-50%) scale(0.8);
  background-color: #fff;
  padding: 0 .2em;
  color: #2196f3;
}




.switch-base {
  margin-top: 20px;
  display: flex;
}

.switch-base span {
  padding-right: 20px;
}


.plane-switch {
  --dot: #fff;
  --street: #6B6D76;
  --street-line: #A8AAB4;
  --street-line-mid: #C0C2C8;
  --sky-1: #60A7FA;
  --sky-2: #2F8EFC;
  --light-1: rgba(255, 233, 0, 1);
  --light-2: rgba(255, 233, 0, .3);
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}

.plane-switch input {
  display: none;
}

.plane-switch input+div {
  -webkit-mask-image: -webkit-radial-gradient(white, black);
  position: relative;
  overflow: hidden;
  width: 50px;
  height: 25px;
  padding: 1px;
  border-radius: 13px;
  background: linear-gradient(90deg, var(--street) 0%, var(--street) 25%, var(--sky-1) 75%, var(--sky-2) 100%) left var(--p, 0%) top 0;
  background-position-x: var(--p, 0%);
  background-size: 400% auto;
  transition: background-position 0.6s;
}

.plane-switch input+div:before,
.plane-switch input+div:after {
  content: "";
  display: block;
  position: absolute;
  transform: translateX(var(--s, 0));
  transition: transform 0.3s;
}

.plane-switch input+div:before {
  width: 42px;
  right: 2px;
  top: 4px;
  height: 1px;
  background: var(--street-line);
  box-shadow: 0 16px 0 0 var(--street-line);
}

.plane-switch input+div:after {
  width: 2px;
  height: 2px;
  border-radius: 50%;
  left: 23px;
  top: 1px;
  -webkit-animation: lights2 2s linear infinite;
  animation: lights2 2s linear infinite;
  box-shadow: inset 0 0 0 2px var(--light-1), 0 21px 0 var(--light-1), 8px 0 0 var(--light-2), 8px 21px 0 var(--light-2), 16px 0 0 var(--light-2), 16px 21px 0 var(--light-2);
}

.plane-switch input+div span {
  display: block;
  position: absolute;
}

.plane-switch input+div span.street-middle {
  top: 12px;
  left: 21px;
  width: 3px;
  height: 1px;
  transform: translateX(var(--s, 0));
  background: var(--street-line-mid);
  box-shadow: 5px 0 0 var(--street-line-mid), 10px 0 0 var(--street-line-mid), 15px 0 0 var(--street-line-mid), 20px 0 0 var(--street-line-mid), 25px 0 0 var(--street-line-mid);
  transition: transform 0.3s;
}

.plane-switch input+div span.cloud {
  width: 12px;
  height: 4px;
  border-radius: 2px;
  background: #fff;
  position: absolute;
  top: var(--ct, 8px);
  left: 100%;
  opacity: var(--co, 0);
  transition: opacity 0.3s;
  -webkit-animation: clouds2 2s linear infinite var(--cd, 0s);
  animation: clouds2 2s linear infinite var(--cd, 0s);
}

.plane-switch input+div span.cloud:before,
.plane-switch input+div span.cloud:after {
  content: "";
  position: absolute;
  transform: translateX(var(--cx, 0));
  border-radius: 50%;
  width: var(--cs, 5px);
  height: var(--cs, 5px);
  background: #fff;
  bottom: 1px;
  left: 1px;
}

.plane-switch input+div span.cloud:after {
  --cs: 6px;
  --cx: 4px;
}

.plane-switch input+div span.cloud.two {
  --ct: 20px;
  --cd: 1s;
  opacity: var(--co-2, 0);
}

.plane-switch input+div div {
  display: table;
  position: relative;
  z-index: 1;
  padding: 5px;
  border-radius: 50%;
  background: var(--dot);
  transform: translateX(var(--x, 0));
  transition: transform 0.6s cubic-bezier(0.2, 0.8, 0.35, 1.2);
}

.plane-switch input+div div svg {
  width: 13px;
  height: 13px;
  display: block;
  color: var(--c, var(--street));
  transition: color 0.6s;
}

.plane-switch input:checked+div {
  --p: 100%;
  --x: 25px;
  --s: -50px;
  --c: var(--sky-2);
  --co: .8;
  --co-2: .6;
}

@keyframes lights2 {

  20%,
  30% {
    box-shadow: inset 0 0 0 2px var(--light-2), 0 21px 0 var(--light-2), 8px 0 0 var(--light-1), 8px 21px 0 var(--light-1), 16px 0 0 var(--light-2), 16px 21px 0 var(--light-2);
  }

  55%,
  65% {
    box-shadow: inset 0 0 0 2px var(--light-2), 0 21px 0 var(--light-2), 8px 0 0 var(--light-2), 8px 21px 0 var(--light-2), 16px 0 0 var(--light-1), 16px 21px 0 var(--light-1);
  }

  90%,
  100% {
    box-shadow: inset 0 0 0 2px var(--light-1), 0 21px 0 var(--light-1), 8px 0 0 var(--light-2), 8px 21px 0 var(--light-2), 16px 0 0 var(--light-2), 16px 21px 0 var(--light-2);
  }
}

@keyframes clouds2 {
  97% {
    transform: translateX(-72px);
    visibility: visible;
  }

  98%,
  100% {
    visibility: hidden;
  }

  99% {
    transform: translateX(-72px);
  }

  100% {
    transform: translateX(0);
  }
}



.save-box {
  margin-top: 20px;
}

.button {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 8px 12px 8px 16px;
  gap: 8px;
  height: 40px;
  width: 128px;
  border: none;
  background: #056bfa27;
  border-radius: 20px;
  cursor: pointer;
}

.lable {
  margin-top: 1px;
  font-size: 19px;
  line-height: 22px;
  color: #056dfa;
  letter-spacing: 1px;
}

.button:hover {
  background: #056bfa49;
}
</style>
