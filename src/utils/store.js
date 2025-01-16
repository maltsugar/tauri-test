import { Store } from "@tauri-apps/plugin-store";

const path = "settings.json";
const key = "sss-key";

export const saveValue = async (value) => {
  const store = await Store.load(path);
  await store.set(key, value);
  await store.save();
};

export const getValue = async () => {
  const store = await Store.load(path);
  const val = await store.get(key);
  return val;
};

export const test = async () => {
  const store = await Store.load("settings.json");

  await store.set("some-key", { value: 568 });

  const val = await store.get("some-key");

  if (val) {
    console.log(val);
  } else {
    console.log("val is null");
  }
};
