# Unrailed孬种分析器

用于分析Unrailed Time模式种子，根据用户输入筛选出符合条件的种子

## 架构

网页使用原生js+html+css编写，种子分析部分使用了webassembly进行加速。
webassembly部分使用rust编写，使用了wasm-pack进行打包

## 原理
Unrailed种子包含一个uint32作为随机数生成器种子，剩下的信息存储了关卡难度与游戏模式。
针对不同模式的生成机制不同， 此工具仅工作与Quick模式，因此最多有2^32*5(个难度)个种子

用户给定地形与需要的车厢后，浏览器会多线程进行计算，并筛选出符合条件的种子。
UI会在选择时显示出符合当前条件种子出现的概率，注意地形，车厢，任务的生成均可视为独立事件。

由于任务的生成顺序取决于车厢的选择顺序以及地形的实际生成顺序，不作筛选使用。但生成机制为一轮一轮生成，大概思路如下：
首先从任务池中枚举出当前能做的任务（任务池大概20～30个，不能做的任务指类似“没有炸弹车厢就不能有使用炸弹的任务”）
然后从任务池中随机选取一个任务，将其从任务池中移除，然后从车厢池中随机选取一个车厢，将其从当前池中删除
下一轮由于会更换地形，所以有些任务会进一步被消除，也有些会被加入。但在任务池空掉前，不会有重复的任务。

## 开发环境(macos)

使用了wasm-pack进行打包，需要安装rust与wasm-pack

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
```

使用--target web 进行编译来直接生成浏览器能使用的js文件
```bash
wasm-pack build --target web
```
### Testing

使用wasm_bindgen_test进行测试，需要安装chromedriver

```bash
brew install --cask chromedriver
```

进行测试
```bash
wasm-pack test --chrome --headless
```

### Deployment
使用任何http server即可部署，开发时我用的是Jetbrain RustRover自带的Server