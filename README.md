# rust-bot
三明治机器人、最早是基于 [subway](https://github.com/libevm/subway) 的 Node.js 版本思路，做了一套基于 BSC PancakeSwap 的程序， 成功记录百分之 30. 最后通过一点点深入发现代码能影响一次夹单很低，主要是节点的问题。 但是 Node.js 垃圾回收，如果在夹单过程中，V8释放内存会导致延迟，所以基于原有代码改造了一份 rust 版本的，在学习过程中现在行情毕竟卷， 想靠这个赚钱大概率跑不通，顶级夹子机器人， 一套代码是通用所有 EVM 链的。 只要是基于Uniswap Fork 的 Dex 交易所即可用，而且他们的节点有可能是跟矿池节点或验证节点合作的，消息是不过公共交易池的，会非常快。这次重构支持多 EVM 链的 AMM 的 Dex 基于配置化启动, 然后整体夹单代码思路已经实现，发送交易暂未实现。

## 环境配置文件 .env
``` shell
RPC_URL_WSS: ws 节点 用户监听交易池、发送交易
SWAP_ROUTER_ADDRESS: 监听目标 swap 发出的交易
SWAP_FACTORY_ADDRESS: swap 工厂合约
SAWP_INIT_CODE_HASH: 创世秘钥
```
所有基于 ETH 的二层网络 API 基本是一致的， 所有代码能通用。
## 运行&打包
``` rust
# dev start
cargo run 
# prod build
cargo build --release
```
