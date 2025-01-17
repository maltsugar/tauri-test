// 导入系统托盘
import { TrayIcon } from "@tauri-apps/api/tray";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Menu } from "@tauri-apps/api/menu";
import { defaultWindowIcon } from '@tauri-apps/api/app';

import { exit, relaunch } from "@tauri-apps/plugin-process";


/**
 * 在这里你可以添加一个托盘菜单，标题，工具提示，事件处理程序等
 */
const options = {
  // icon 的相对路径基于：项目根目录/src-tauri/
  // icon: "icons/32x32.png",
  tooltip: "全盘搜工具",
  // 是否在左键点击时显示托盘菜单，默认为 true。当然不能为 true 啦，程序进入后台不得左键点击图标显示窗口啊。
  menuOnLeftClick: false,
  // 托盘图标上事件的处理程序。
  action: (event) => {
    // 左键点击事件
    if (
      event.type === "Click" &&
      event.button === "Left" &&
      event.buttonState === "Down"
    ) {
      console.log("单击事件");
      // 显示窗口
      winShowFocus();
    }
  },
};
/**
 * 窗口置顶显示
 */
async function winShowFocus() {
  // 获取窗体实例
  const win = getCurrentWindow();
  // 检查窗口是否见，如果不可见则显示出来
  if (!(await win.isVisible())) {
    win.show();
  } else {
    // 检查是否处于最小化状态，如果处于最小化状态则解除最小化
    if (await win.isMinimized()) {
      await win.unminimize();
    }
    // 窗口置顶
    await win.setFocus();
  }
}

/**
 * 创建托盘菜单
 */
async function createMenu() {
  return await Menu.new({
    // items 的显示顺序是倒过来的
    items: [
      {
        id: "show",
        text: "显示窗口",
        action: () => {
          winShowFocus();
        },
      },
      {
        id: "quit",
        text: "退出",
        action: async () => {
          console.log("退出应用");
          await exit(0);
        },
      },
    ],
  });
}




/**
 * 创建系统托盘
 */
const trayIdKey = "__trayIdKey";
export async function createTray() {

  const trayId = sessionStorage.getItem(trayIdKey)
  if (trayId) {
    console.log('trayId',trayId)
    TrayIcon.removeById(trayId)
  }

  options.icon = await defaultWindowIcon();
  options.menu = await createMenu();
  const tray = await TrayIcon.new(options);
  console.log('tray',tray)
  sessionStorage.setItem(trayIdKey, tray.id)

}

