import { enable, isEnabled, disable } from '@tauri-apps/plugin-autostart'

await enable()

console.log(`registered for autostart? ${await isEnabled()}`)

disable()