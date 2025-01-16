import { fetch } from "@tauri-apps/plugin-http";
import { ref } from "vue";

const token = ref();
const BASE_URL = "https://so.yuneu.com";



const http = (path, params, method = "POST") => {
  const headers = {
    accept: "application/json, text/plain, */*",
    "accept-language": "zh-CN,zh;q=0.9,en;q=0.8",
    "content-type": "application/json",
    "user-agent":
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0",
    "sec-fetch-dest": "empty",
    "sec-fetch-mode": "cors",
    "sec-fetch-site": "same-origin",
  };

  if (token.value) {
    headers["authorization"] = `Bearer ${token.value}`;
  }

  console.log("headers", headers);

  const url = `${BASE_URL}${path}`;

  const config = {
    method,
    headers,
    body: JSON.stringify(params),
  };
  if (method == "GET") {
    delete config.body;
  }

  return fetch(url, config).then((res) => {
    if (res.ok) {
      return res.json();
    }
    throw new Error("Network response was not ok.");
  });
};

export { token };
export default http;
